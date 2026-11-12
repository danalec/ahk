//! The PNG reader of the box.
//!
//! The captures of a compositor are PNG files, and a script asks two things
//! of one: the colour of a pixel, and where a colour is. Both are the same
//! read -- the file becomes a grid of eight-bit samples -- and the two
//! questions are asked of that grid afterwards.
//!
//! The format is read from its own specification, and the layout is the one
//! it publishes: an eight-byte signature, then chunks, each with a length, a
//! type, the bytes and a CRC-32 of the type and the bytes. The chunks this
//! reader needs are `IHDR` (the size, the kind of the pixel and the
//! interlacing), `PLTE` (the palette of an indexed image), `IDAT` and `IEND`.
//! A chunk it does not know is skipped, after its CRC is checked: a decoder
//! that skipped a chunk it could not check would read a file that is damaged
//! and call it an image.
//!
//! The bytes of the image are a zlib stream, and the stream is read by
//! `rgx-core`, which is the one dependency of the box that this reader leans
//! on. What is here is the format around that stream: the filtering of the
//! rows (the five filters of the specification, applied in place), the
//! unfiltering -- which is the same five, walked backwards -- and the seven
//! passes of an interlaced image.
//!
//! Nothing here trusts the file. A length that runs past the end of it, a
//! CRC that does not match, a pixel kind this reader does not carry, a
//! palette that is too short for the index it is asked for: each of them is
//! an error with the reason, and never a guess.

use crate::picture::crc32::crc32;

/// The eight bytes every PNG begins with.
const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];

/// How the samples of a pixel are laid out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    /// One number of grey, `depth` bits.
    Grey,
    /// A colour of three numbers, `depth` bits each.
    Rgb,
    /// A number of grey and its transparency.
    GreyAlpha,
    /// A colour and its transparency.
    Rgba,
    /// A place in the palette that `PLTE` carries.
    Indexed,
}

impl Colour {
    /// The number of the samples of a pixel.
    pub fn samples(self) -> usize {
        match self {
            Colour::Grey | Colour::Indexed => 1,
            Colour::GreyAlpha => 2,
            Colour::Rgb => 3,
            Colour::Rgba => 4,
        }
    }
}

/// An image that was read out of a file.
#[derive(Debug)]
pub struct Image {
    /// The width, in pixels.
    pub width: usize,
    /// The height, in pixels.
    pub height: usize,
    /// The samples of the image, row by row, with no padding between rows.
    ///
    /// The samples are eight bits each: the reader expands a shallower depth
    /// on its way in, so that the answer has one shape.
    pub samples: Vec<u8>,
    /// The palette of an indexed image, three bytes a place.
    pub palette: Vec<u8>,
    /// How the samples are laid out.
    pub colour: Colour,
}

impl Image {
    /// The colour of a pixel, as three numbers of eight bits.
    pub fn colour_at(&self, x: usize, y: usize) -> Option<(u8, u8, u8)> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let samples = self.colour.samples();
        let offset = (y * self.width + x) * samples;
        match self.colour {
            Colour::Grey => self.samples.get(offset).map(|v| (*v, *v, *v)),
            Colour::GreyAlpha => self.samples.get(offset).map(|v| (*v, *v, *v)),
            Colour::Rgb | Colour::Rgba => {
                let bytes = self.samples.get(offset..offset + 3)?;
                Some((bytes[0], bytes[1], bytes[2]))
            }
            Colour::Indexed => {
                let place = *self.samples.get(offset)? as usize * 3;
                let bytes = self.palette.get(place..place + 3)?;
                Some((bytes[0], bytes[1], bytes[2]))
            }
        }
    }
}

/// Read a PNG file.
pub fn read(bytes: &[u8]) -> Result<Image, String> {
    if bytes.len() < 8 || bytes[..8] != SIGNATURE {
        return Err("the file is not a PNG".to_string());
    }
    let mut at = 8;
    let mut header: Option<Header> = None;
    let mut palette: Vec<u8> = Vec::new();
    let mut data: Vec<u8> = Vec::new();
    while at + 8 <= bytes.len() {
        let length =
            u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]) as usize;
        let kind = &bytes[at + 4..at + 8];
        let start = at + 8;
        let end = start
            .checked_add(length)
            .ok_or_else(|| "the chunk is longer than the file".to_string())?;
        if end + 4 > bytes.len() {
            return Err("the chunk is longer than the file".to_string());
        }
        let body = &bytes[start..end];
        // The CRC of a chunk covers the type and the bytes, and not the
        // length: a damaged file is refused rather than read.
        let stored =
            u32::from_be_bytes([bytes[end], bytes[end + 1], bytes[end + 2], bytes[end + 3]]);
        let mut covered = Vec::with_capacity(4 + body.len());
        covered.extend_from_slice(kind);
        covered.extend_from_slice(body);
        if crc32(&covered) != stored {
            return Err(format!(
                "the chunk {} is damaged: its CRC is not the one of its bytes",
                String::from_utf8_lossy(kind)
            ));
        }
        match kind {
            b"IHDR" => header = Some(Header::read(body)?),
            b"PLTE" => palette = body.to_vec(),
            b"IDAT" => data.extend_from_slice(body),
            b"IEND" => break,
            _ => {}
        }
        at = end + 4;
    }
    let header = header.ok_or_else(|| "the file has no header".to_string())?;
    if data.is_empty() {
        return Err("the file has no image data".to_string());
    }
    let raw = crate::picture::zlib::decompress(&data)
        .map_err(|error| format!("the image data is not a zlib stream: {error}"))?;
    let samples = the_samples(&header, &raw)?;
    Ok(Image {
        width: header.width,
        height: header.height,
        samples,
        palette,
        colour: header.colour,
    })
}

/// The header of a file.
struct Header {
    width: usize,
    height: usize,
    depth: u8,
    colour: Colour,
    interlace: bool,
}

impl Header {
    fn read(body: &[u8]) -> Result<Header, String> {
        if body.len() < 13 {
            return Err("the header is shorter than the format says".to_string());
        }
        let width = u32::from_be_bytes([body[0], body[1], body[2], body[3]]) as usize;
        let height = u32::from_be_bytes([body[4], body[5], body[6], body[7]]) as usize;
        if width == 0 || height == 0 {
            return Err("the image is empty".to_string());
        }
        let depth = body[8];
        let colour = match body[9] {
            0 => Colour::Grey,
            2 => Colour::Rgb,
            3 => Colour::Indexed,
            4 => Colour::GreyAlpha,
            6 => Colour::Rgba,
            other => {
                return Err(format!(
                    "the kind of the pixel is {other}, which is not a kind of PNG"
                ));
            }
        };
        let allowed: &[u8] = match colour {
            Colour::Grey => &[1, 2, 4, 8, 16],
            Colour::Indexed => &[1, 2, 4, 8],
            _ => &[8, 16],
        };
        if !allowed.contains(&depth) {
            return Err(format!(
                "a depth of {depth} bits is not one of this kind of pixel"
            ));
        }
        if body[10] != 0 {
            return Err("the compression of the image is not the one of PNG".to_string());
        }
        if body[11] != 0 {
            return Err("the filter of the image is not the one of PNG".to_string());
        }
        let interlace = match body[12] {
            0 => false,
            1 => true,
            other => return Err(format!("the interlacing {other} is not a method of PNG")),
        };
        Ok(Header {
            width,
            height,
            depth,
            colour,
            interlace,
        })
    }

    /// How many bytes a row of a pass takes, without the filter byte.
    fn stride(&self, width: usize) -> usize {
        let bits = width * self.colour.samples() * self.depth as usize;
        bits.div_ceil(8)
    }

    /// How many bytes a sample of the row takes. A depth below eight packs
    /// several samples into a byte, and the reader expands those on the way
    /// out, so the stride is counted in bytes of the file.
    fn bytes_a_sample(&self) -> usize {
        (self.depth as usize).div_ceil(8)
    }
}

/// The seven passes of an interlaced image, as `(x step, y step, x offset, y offset)`.
const PASSES: [(usize, usize, usize, usize); 7] = [
    (8, 8, 0, 0),
    (8, 8, 4, 0),
    (4, 8, 0, 4),
    (4, 4, 2, 0),
    (2, 4, 0, 2),
    (2, 2, 1, 0),
    (1, 2, 0, 1),
];

/// The samples of the image, unfiltered and laid out by pixel.
fn the_samples(header: &Header, raw: &[u8]) -> Result<Vec<u8>, String> {
    let samples = header.colour.samples();
    let bytes = header.bytes_a_sample();
    let mut out = vec![0u8; header.width * header.height * samples * bytes];
    let mut at = 0;
    let passes: &[(usize, usize, usize, usize)] = if header.interlace {
        &PASSES
    } else {
        &[(1, 1, 0, 0)]
    };
    for (step_x, step_y, offset_x, offset_y) in passes {
        // The size of the pass: the pixels of the image it carries.
        let width = header.width.saturating_sub(*offset_x).div_ceil(*step_x);
        let height = header.height.saturating_sub(*offset_y).div_ceil(*step_y);
        if width == 0 || height == 0 {
            continue;
        }
        let stride = header.stride(width);
        let mut previous = vec![0u8; stride];
        let mut row = vec![0u8; stride];
        for y in 0..height {
            if at + 1 + stride > raw.len() {
                return Err("the image data ends before the image does".to_string());
            }
            let filter = raw[at];
            row.copy_from_slice(&raw[at + 1..at + 1 + stride]);
            at += 1 + stride;
            unfilter(filter, &mut row, &previous, bytes)
                .map_err(|reason| format!("the filter {reason}"))?;
            // The row goes into the image at the places of this pass. A
            // depth of eight or more has the bytes of a sample where its
            // pixel is; a shallower one packs several pixels into a byte,
            // and that row is copied whole and expanded afterwards.
            if header.depth < 8 {
                let target = offset_y + y * step_y;
                let start = target * stride;
                if let Some(slot) = out.get_mut(start..start + stride) {
                    slot.copy_from_slice(&row);
                }
            } else {
                for x in 0..width {
                    let source = x * samples * bytes;
                    let target_x = offset_x + x * step_x;
                    let target_y = offset_y + y * step_y;
                    let target = (target_y * header.width + target_x) * samples * bytes;
                    if let Some(bytes_of_the_row) = row.get(source..source + samples * bytes) {
                        for (index, value) in bytes_of_the_row.iter().enumerate() {
                            if let Some(slot) = out.get_mut(target + index) {
                                *slot = *value;
                            }
                        }
                    }
                }
            }
            previous.copy_from_slice(&row);
        }
    }
    // A depth below eight packs several samples into a byte, and the answer
    // is one sample to a byte.
    if header.depth < 8 {
        out = expand(header, &out);
    }
    Ok(out)
}

/// Turn the packed samples of a shallow image into one sample a byte.
fn expand(header: &Header, packed: &[u8]) -> Vec<u8> {
    let samples = header.colour.samples();
    let depth = header.depth as usize;
    let per_byte = 8 / depth;
    let mask = (1u16 << depth) - 1;
    let width = header.width;
    let mut out = vec![0u8; width * header.height * samples];
    for y in 0..header.height {
        let row = &packed[y * header.stride(width)..(y + 1) * header.stride(width)];
        for x in 0..width {
            for s in 0..samples {
                let place = x * samples + s;
                let byte = row[place / per_byte];
                let shift = 8 - depth * (place % per_byte + 1);
                let value = (byte as u16 >> shift) & mask;
                // A sample of a shallow depth counts as far as it can: one
                // bit of white is 255 and not 1, and the four bits of a
                // grey are the sixteen steps the format says they are.
                let scale = 255 / mask as usize;
                out[(y * width + x) * samples + s] = (value as usize * scale) as u8;
            }
        }
    }
    out
}

/// Undo one of the five filters of the rows.
///
/// The filters are the ones of the specification, and every one of them is
/// read from the bytes before it: the first byte of the row is the one of the
/// row itself, and the `bpp` is how far back the sample of the same place in
/// the pixel above is.
fn unfilter(kind: u8, row: &mut [u8], previous: &[u8], bpp: usize) -> Result<(), u8> {
    match kind {
        0 => Ok(()),
        1 => {
            for index in bpp..row.len() {
                row[index] = row[index].wrapping_add(row[index - bpp]);
            }
            Ok(())
        }
        2 => {
            for index in 0..row.len() {
                row[index] = row[index].wrapping_add(previous[index]);
            }
            Ok(())
        }
        3 => {
            for index in 0..row.len() {
                let left = if index >= bpp {
                    row[index - bpp] as u16
                } else {
                    0
                };
                let up = previous[index] as u16;
                row[index] = row[index].wrapping_add(((left + up) / 2) as u8);
            }
            Ok(())
        }
        4 => {
            for index in 0..row.len() {
                let left = if index >= bpp { row[index - bpp] } else { 0 };
                let up = previous[index];
                let up_left = if index >= bpp {
                    previous[index - bpp]
                } else {
                    0
                };
                row[index] = row[index].wrapping_add(paeth(left, up, up_left));
            }
            Ok(())
        }
        other => Err(other),
    }
}

/// Write an RGB PNG of the pixels that a capture holds.
///
/// The X11 backend captures the screen as raw RGB bytes, and a `Screenshot`
/// asks for a file. The format of the file is the same one the reader of this
/// module reads: an `IHDR` of colour two, one `IDAT` of a zlib stream, and an
/// `IEND`; the CRC of each chunk is the one of the bytes it covers.
///
/// The zlib stream carries stored deflate blocks, which the format allows and
/// every decoder reads. A real deflate encoder would be a few hundred lines
/// spent on a file that is written once and handed to a script; stored blocks
/// cost size and nothing else, and the size is bounded by the screen.
pub fn write_rgb(width: usize, height: usize, pixels: &[u8]) -> Result<Vec<u8>, String> {
    if width == 0 || height == 0 {
        return Err("the capture is empty".to_string());
    }
    if width
        .checked_mul(height)
        .and_then(|count| count.checked_mul(3))
        != Some(pixels.len())
    {
        return Err("the pixels are not the size of the image".to_string());
    }
    let mut out = Vec::with_capacity(pixels.len() + height + 64);
    out.extend_from_slice(&SIGNATURE);
    let mut header = Vec::with_capacity(13);
    header.extend_from_slice(&(width as u32).to_be_bytes());
    header.extend_from_slice(&(height as u32).to_be_bytes());
    header.extend_from_slice(&[8, 2, 0, 0, 0]);
    write_chunk(&mut out, b"IHDR", &header);
    // A row of the image, with the filter byte of `None` in front of it.
    let mut rows = Vec::with_capacity(height * (1 + width * 3));
    for y in 0..height {
        rows.push(0);
        rows.extend_from_slice(&pixels[y * width * 3..(y + 1) * width * 3]);
    }
    let compressed = crate::picture::zlib::compress(&rows);
    write_chunk(&mut out, b"IDAT", &compressed);
    write_chunk(&mut out, b"IEND", &[]);
    Ok(out)
}

/// Write one chunk, with the CRC of its type and its bytes.
fn write_chunk(out: &mut Vec<u8>, kind: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(body);
    let mut covered = Vec::with_capacity(4 + body.len());
    covered.extend_from_slice(kind);
    covered.extend_from_slice(body);
    out.extend_from_slice(&crc32(&covered).to_be_bytes());
}

/// The prediction of the `Paeth` filter.
fn paeth(left: u8, up: u8, up_left: u8) -> u8 {
    let p = left as i16 + up as i16 - up_left as i16;
    let a = (p - left as i16).abs();
    let b = (p - up as i16).abs();
    let c = (p - up_left as i16).abs();
    if a <= b && a <= c {
        left
    } else if b <= c {
        up
    } else {
        up_left
    }
}

#[cfg(test)]
mod tests {
    use super::{paeth, read};

    /// The file that is not a PNG is refused with the reason.
    #[test]
    fn a_file_that_is_not_a_png_is_refused() {
        assert!(read(b"not a png at all").is_err());
    }

    /// The predictor of `Paeth` chooses the sample the sum points at.
    #[test]
    fn the_predictor_of_paeth_is_the_nearest_of_the_three() {
        assert_eq!(paeth(10, 10, 10), 10);
        assert_eq!(paeth(0, 0, 0), 0);
        // The left is the nearest of the three.
        assert_eq!(paeth(100, 0, 0), 100);
        // The one above is the nearest.
        assert_eq!(paeth(0, 100, 0), 100);
    }
}
