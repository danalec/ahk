//! The zlib stream of a PNG, read and written.
//!
//! The image data of a PNG is a zlib stream (RFC 1950): a two-byte header, a
//! DEFLATE body (RFC 1951) and a four-byte Adler-32 trailer. The reader below
//! is the DEFLATE decoder, written from the description of the format rather
//! than from the source of another implementation, and the writer emits
//! *stored* blocks -- the one block kind that carries its bytes as they are.
//!
//! # Why the writer does not compress
//!
//! A stored block is a valid DEFLATE block, and a zlib stream of stored blocks
//! is a valid zlib stream, so every decoder reads it. A real encoder is the
//! Huffman machinery and the match finder, a few hundred lines, spent on a
//! file that is written once and handed to a script. What it costs is size,
//! and the size is bounded by the screen: a 1920×1080 capture is about six
//! megabytes the way a screenshot is six megabytes of pixels.
//!
//! # The reader treats its input as untrusted
//!
//! A PNG arrives from a file, and the file may be damaged or hostile. A
//! length that runs past the end of the stream, a Huffman code that cannot be
//! built, a distance that reaches further back than the output, a block kind
//! the format does not define: each of them is an error with the reason, and
//! never a guess and never an allocation the file asked for.

use std::vec;
use std::vec::Vec;

/// The largest value a stored block can carry.
const STORED_LIMIT: usize = 0xFFFF;

/// The reason a stream could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InflateError {
    /// What went wrong, in a sentence.
    pub reason: &'static str,
}

impl std::fmt::Display for InflateError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.reason)
    }
}

impl std::error::Error for InflateError {}

/// Read a zlib stream: the header, the DEFLATE body and the trailer.
pub fn decompress(stream: &[u8]) -> Result<Vec<u8>, InflateError> {
    if stream.len() < 2 {
        return Err(the_reason("the stream is shorter than the header of zlib"));
    }
    let (cmf, flg) = (stream[0], stream[1]);
    // The method is eight: DEFLATE, and there is no other.
    if cmf & 0x0F != 8 {
        return Err(the_reason("the compression method is not deflate"));
    }
    // The window is at most seven, and the two header bytes are a multiple of
    // thirty-one: that is the check the format prescribes.
    if cmf >> 4 > 7 {
        return Err(the_reason(
            "the window of the stream is larger than zlib allows",
        ));
    }
    if (u16::from(cmf) * 256 + u16::from(flg)) % 31 != 0 {
        return Err(the_reason("the header of the stream does not check"));
    }
    if flg & 0x20 != 0 {
        return Err(the_reason(
            "the stream names a dictionary, which a PNG does not carry",
        ));
    }
    let body = &stream[2..];
    let (output, used) = inflate(body)?;
    // The trailer is the Adler-32 of what was read, and a stream that carries
    // a different one is a stream that was damaged on its way here.
    if used + 4 <= body.len() {
        let stored =
            u32::from_be_bytes([body[used], body[used + 1], body[used + 2], body[used + 3]]);
        if stored != adler32(&output) {
            return Err(the_reason(
                "the checksum of the stream is not the one of its bytes",
            ));
        }
    }
    Ok(output)
}

/// Read a DEFLATE body, and answer what it holds and how many bytes it took.
fn inflate(input: &[u8]) -> Result<(Vec<u8>, usize), InflateError> {
    let mut reader = Bits::new(input);
    let mut output: Vec<u8> = Vec::new();
    loop {
        let last = reader.take(1)? == 1;
        match reader.take(2)? {
            0 => a_stored_block(&mut reader, &mut output)?,
            1 => the_fixed_block(&mut reader, &mut output)?,
            2 => the_dynamic_block(&mut reader, &mut output)?,
            _ => return Err(the_reason("the block kind is not one the format defines")),
        }
        if last {
            break;
        }
    }
    reader.align();
    Ok((output, reader.bytes_read()))
}

/// A block whose bytes are the output.
fn a_stored_block(reader: &mut Bits<'_>, output: &mut Vec<u8>) -> Result<(), InflateError> {
    reader.align();
    let length = reader.take(16)? as usize;
    let complement = reader.take(16)? as usize;
    if length != !complement & 0xFFFF {
        return Err(the_reason(
            "the length of a stored block does not match its complement",
        ));
    }
    let bytes = reader.bytes(length)?;
    output.extend_from_slice(bytes);
    Ok(())
}

/// The block whose codes are the ones the format fixes.
fn the_fixed_block(reader: &mut Bits<'_>, output: &mut Vec<u8>) -> Result<(), InflateError> {
    let literals = the_fixed_literal_code();
    let distances = the_fixed_distance_code();
    the_codes(reader, output, &literals, &distances)
}

/// A block whose codes are described in the block itself.
fn the_dynamic_block(reader: &mut Bits<'_>, output: &mut Vec<u8>) -> Result<(), InflateError> {
    let literals_count = reader.take(5)? as usize + 257;
    let distances_count = reader.take(5)? as usize + 1;
    let lengths_count = reader.take(4)? as usize + 4;
    if literals_count > 286 || distances_count > 30 {
        return Err(the_reason(
            "the block describes more codes than the format allows",
        ));
    }
    // The lengths of the codes of the lengths are given in the order the
    // format fixes, and not in the order of the alphabet.
    let order: [usize; 19] = [
        16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
    ];
    let mut lengths = [0u8; 19];
    for place in 0..lengths_count {
        lengths[order[place]] = reader.take(3)? as u8;
    }
    let length_code = Code::build(&lengths)?;
    // The lengths of the literals and of the distances, run-length coded.
    let mut all = vec![0u8; literals_count + distances_count];
    let mut place = 0;
    while place < all.len() {
        let symbol = length_code.read(reader)?;
        match symbol {
            0..=15 => {
                all[place] = symbol as u8;
                place += 1;
            }
            16 => {
                if place == 0 {
                    return Err(the_reason("a repeat of a length has nothing to repeat"));
                }
                let previous = all[place - 1];
                let count = 3 + reader.take(2)? as usize;
                if place + count > all.len() {
                    return Err(the_reason("a repeat of a length runs past the codes"));
                }
                for _ in 0..count {
                    all[place] = previous;
                    place += 1;
                }
            }
            17 => {
                let count = 3 + reader.take(3)? as usize;
                if place + count > all.len() {
                    return Err(the_reason("a run of zeros runs past the codes"));
                }
                place += count;
            }
            18 => {
                let count = 11 + reader.take(7)? as usize;
                if place + count > all.len() {
                    return Err(the_reason("a run of zeros runs past the codes"));
                }
                place += count;
            }
            _ => {
                return Err(the_reason(
                    "the block uses a length the format does not define",
                ));
            }
        }
    }
    let literals = Code::build(&all[..literals_count])?;
    let distances = Code::build(&all[literals_count..])?;
    the_codes(reader, output, &literals, &distances)
}

/// The lengths a match of a given code takes, and the extra bits it carries.
const LENGTH_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LENGTH_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DISTANCE_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DISTANCE_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

/// Read the symbols of a block, with the codes it gave.
fn the_codes(
    reader: &mut Bits<'_>,
    output: &mut Vec<u8>,
    literals: &Code,
    distances: &Code,
) -> Result<(), InflateError> {
    loop {
        let symbol = literals.read(reader)?;
        match symbol {
            0..=255 => output.push(symbol as u8),
            256 => return Ok(()),
            257..=285 => {
                let index = (symbol - 257) as usize;
                let length =
                    LENGTH_BASE[index] as usize + reader.take(LENGTH_EXTRA[index])? as usize;
                let code = distances.read(reader)? as usize;
                if code >= 30 {
                    return Err(the_reason(
                        "the block names a distance the format does not define",
                    ));
                }
                let distance =
                    DISTANCE_BASE[code] as usize + reader.take(DISTANCE_EXTRA[code])? as usize;
                if distance == 0 || distance > output.len() {
                    return Err(the_reason(
                        "the match reaches further back than the output goes",
                    ));
                }
                // The match is copied a byte at a time on purpose: it may
                // overlap itself, and a `copy_within` of an overlapping range
                // is not the answer the format asks for.
                let start = output.len() - distance;
                for step in 0..length {
                    let byte = output[start + step];
                    output.push(byte);
                }
            }
            _ => {
                return Err(the_reason(
                    "the block names a length the format does not define",
                ));
            }
        }
    }
}

/// A Huffman code, as the canonical code the format describes.
///
/// The code is kept as the canonical lengths and the first code of each
/// length, which is the shape the format defines. Reading a symbol walks the
/// bits one at a time: the bits arrive least significant first, so the value
/// is built by appending each bit to the right, and the first length whose
/// range holds the value is the symbol.
struct Code {
    /// How many codes have each length: `counts[bits]` is the number of codes
    /// of `bits` bits.
    counts: [u16; 16],
    /// The symbols, in the order the canonical code numbers them.
    symbols: Vec<u16>,
}

impl Code {
    /// Build the canonical code of the lengths.
    fn build(lengths: &[u8]) -> Result<Code, InflateError> {
        let mut counts = [0u16; 16];
        for length in lengths {
            if *length as usize > 15 {
                return Err(the_reason("a code is longer than the format allows"));
            }
            counts[*length as usize] += 1;
        }
        counts[0] = 0;
        // The symbols, in the order of their codes: the order of the alphabet
        // within a length, and the lengths from the shortest up.
        let mut symbols = Vec::with_capacity(lengths.iter().filter(|l| **l > 0).count());
        for bits in 1..16 {
            for (symbol, length) in lengths.iter().enumerate() {
                if *length as usize == bits {
                    symbols.push(symbol as u16);
                }
            }
        }
        Ok(Code { counts, symbols })
    }

    /// Read one symbol.
    fn read(&self, reader: &mut Bits<'_>) -> Result<u16, InflateError> {
        let mut code = 0u16;
        let mut first = 0u16;
        let mut index = 0u16;
        for bits in 1..=15 {
            code |= reader.take(1)? as u16;
            let count = self.counts[bits];
            if code >= first && code - first < count {
                let place = (index + (code - first)) as usize;
                return self
                    .symbols
                    .get(place)
                    .copied()
                    .ok_or_else(|| the_reason("the stream uses a code it did not describe"));
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
        }
        Err(the_reason("the stream uses a code it did not describe"))
    }
}

/// The code the format fixes for the literals and the lengths.
fn the_fixed_literal_code() -> Code {
    let mut lengths = [0u8; 288];
    for length in lengths.iter_mut().take(144) {
        *length = 8;
    }
    for length in lengths.iter_mut().take(256).skip(144) {
        *length = 9;
    }
    for length in lengths.iter_mut().take(280).skip(256) {
        *length = 7;
    }
    for length in lengths.iter_mut().take(288).skip(280) {
        *length = 8;
    }
    Code::build(&lengths).expect("the code of the format is a code")
}

/// The code the format fixes for the distances.
fn the_fixed_distance_code() -> Code {
    Code::build(&[5u8; 30]).expect("the code of the format is a code")
}

/// The bits of a DEFLATE stream, read from the least significant up.
struct Bits<'a> {
    bytes: &'a [u8],
    /// The byte the next bit comes from.
    at: usize,
    /// How many bits of that byte are already spent.
    spent: u32,
}

impl<'a> Bits<'a> {
    fn new(bytes: &'a [u8]) -> Bits<'a> {
        Bits {
            bytes,
            at: 0,
            spent: 0,
        }
    }

    /// Read the whole number of `count` bits, the least significant first.
    fn take(&mut self, count: u8) -> Result<u32, InflateError> {
        let mut value = 0u32;
        for place in 0..count {
            let byte = *self
                .bytes
                .get(self.at)
                .ok_or_else(|| the_reason("the stream ends before the image does"))?;
            let bit = (byte >> self.spent) & 1;
            value |= u32::from(bit) << place;
            self.spent += 1;
            if self.spent == 8 {
                self.spent = 0;
                self.at += 1;
            }
        }
        Ok(value)
    }

    /// Drop the bits that are left of the byte being read.
    fn align(&mut self) {
        if self.spent != 0 {
            self.spent = 0;
            self.at += 1;
        }
    }

    /// Take the bytes that follow, which must be `count` of them.
    fn bytes(&mut self, count: usize) -> Result<&'a [u8], InflateError> {
        let end = self
            .at
            .checked_add(count)
            .ok_or_else(|| the_reason("the stream asks for more bytes than it has"))?;
        let slice = self
            .bytes
            .get(self.at..end)
            .ok_or_else(|| the_reason("the stream ends before the image does"))?;
        self.at = end;
        Ok(slice)
    }

    /// How many bytes were read.
    fn bytes_read(&self) -> usize {
        self.at
    }
}

/// The Adler-32 of the bytes.
pub fn adler32(bytes: &[u8]) -> u32 {
    let mut first: u32 = 1;
    let mut second: u32 = 0;
    for byte in bytes {
        first = (first + u32::from(*byte)) % 65521;
        second = (second + first) % 65521;
    }
    (second << 16) | first
}

/// Write a zlib stream of stored blocks.
pub fn compress(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bytes.len() + bytes.len() / STORED_LIMIT * 5 + 16);
    // The header: deflate, a window of thirty-two thousand, and the check the
    // format asks for. The level bits are zero, which is what a stored block
    // says.
    out.push(0x78);
    out.push(0x01);
    let mut place = 0;
    loop {
        let count = (bytes.len() - place).min(STORED_LIMIT);
        let last = place + count >= bytes.len();
        out.push(if last { 1 } else { 0 });
        let length = count as u16;
        out.extend_from_slice(&length.to_le_bytes());
        out.extend_from_slice(&(!length).to_le_bytes());
        out.extend_from_slice(&bytes[place..place + count]);
        place += count;
        if last {
            break;
        }
    }
    out.extend_from_slice(&adler32(bytes).to_be_bytes());
    out
}

/// The reason a stream could not be read.
fn the_reason(reason: &'static str) -> InflateError {
    InflateError { reason }
}

#[cfg(test)]
mod tests {
    use super::{adler32, compress, decompress};

    /// A stream is the one of a text everyone knows: the Adler-32 of "abc" is
    /// the value the standard publishes.
    #[test]
    fn the_check_of_the_standard_is_the_one_it_publishes() {
        assert_eq!(adler32(b"abc"), 0x024D_0127);
    }

    /// A text is written and read back, and what comes back is the text.
    #[test]
    fn a_text_is_written_and_read_back() {
        for text in [&b""[..], b"a", b"hello, world", &[0u8; 70000][..]] {
            let stream = compress(text);
            let back = decompress(&stream).expect("the stream is read");
            assert_eq!(back, text, "a text of {} bytes", text.len());
        }
    }

    /// The stream of the format is read: `deflate` of "hello" is the five
    /// bytes of the text after the two bytes of the header, and that is the
    /// stream every PNG carries.
    #[test]
    fn a_stream_of_the_format_is_read() {
        // The zlib stream of "hello", written by another implementation.
        let stream = [
            0x78, 0x9C, 0xCB, 0x48, 0xCD, 0xC9, 0xC9, 0x07, 0x00, 0x06, 0x2C, 0x02, 0x15,
        ];
        assert_eq!(decompress(&stream).expect("the stream is read"), b"hello");
    }

    /// A stream that is too short is refused, with the reason.
    #[test]
    fn a_stream_that_is_too_short_is_refused() {
        assert!(decompress(&[0x78]).is_err());
    }

    /// The header is checked, the way the format says.
    #[test]
    fn a_header_that_does_not_check_is_refused() {
        assert!(decompress(&[0x79, 0x9C, 0x00, 0x00]).is_err());
    }

    /// A stream that was damaged is refused rather than read as something.
    #[test]
    fn a_stream_that_was_damaged_is_refused() {
        let mut stream = compress(b"the bytes of a picture");
        let last = stream.len() - 1;
        stream[last] ^= 0xFF;
        assert!(decompress(&stream).is_err());
    }
}
