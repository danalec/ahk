//! The pictures of the box.
//!
//! The box reads the images of the machine with the `png` crate: it
//! reads the pixels of a file, and it looks for them on the screen with
//! a capture of `grim`.
//!
//! The pictures that are loaded are kept: the number that identifies
//! one is the place of the picture in the table of the desktop, and the
//! tree asks for them by that number.

use std::path::Path;

use ahk::desktop::{Image, ImageOptions, ImageSearch, PictureOptions, Screen};

use crate::desktop::LabDesktop;

/// A picture that was loaded: the red, the green and the blue of every
/// pixel, one after the other.
pub struct Picture {
    /// The pixels of the picture: three bytes for every pixel.
    pub pixels: Vec<u8>,
    /// The width of the picture, in pixels.
    pub width: usize,
    /// The height of the picture, in pixels.
    pub height: usize,
}

/// Read the pixels of a picture of a file.
pub fn read(path: &str) -> Result<Picture, String> {
    let file = std::fs::File::open(Path::new(path)).map_err(|error| error.to_string())?;
    let decoder = png::Decoder::new(std::io::BufReader::new(file));
    let mut reader = decoder.read_info().map_err(|error| error.to_string())?;
    let mut buffer = vec![0u8; reader.output_buffer_size()];
    let info = reader
        .next_frame(&mut buffer)
        .map_err(|error| error.to_string())?;
    let width = info.width as usize;
    let height = info.height as usize;
    let bytes = &buffer[..info.buffer_size()];
    let channels = match info.color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        png::ColorType::Grayscale => 1,
        png::ColorType::GrayscaleAlpha => 2,
        png::ColorType::Indexed => 1,
    };
    let palette = reader.info().palette.clone();
    let mut pixels = Vec::with_capacity(width * height * 3);
    for y in 0..height {
        for x in 0..width {
            let offset = (y * width + x) * channels;
            if offset + channels > bytes.len() {
                break;
            }
            let (red, green, blue) = match info.color_type {
                png::ColorType::Rgb => (bytes[offset], bytes[offset + 1], bytes[offset + 2]),
                png::ColorType::Rgba => (bytes[offset], bytes[offset + 1], bytes[offset + 2]),
                png::ColorType::Grayscale => (bytes[offset], bytes[offset], bytes[offset]),
                png::ColorType::GrayscaleAlpha => (bytes[offset], bytes[offset], bytes[offset]),
                png::ColorType::Indexed => {
                    let Some(palette) = palette.as_ref() else {
                        break;
                    };
                    let at = bytes[offset] as usize * 3;
                    if at + 2 >= palette.len() {
                        break;
                    }
                    (palette[at], palette[at + 1], palette[at + 2])
                }
            };
            pixels.push(red);
            pixels.push(green);
            pixels.push(blue);
        }
    }
    Ok(Picture {
        pixels,
        width,
        height,
    })
}

/// The picture at the size that was asked for.
///
/// The size of v2: a dimension that is left out, or that is zero, is the
/// size the picture has, and a dimension that is minus one is the other
/// one, with the proportion of the picture kept.
pub fn resized(picture: Picture, width: Option<i64>, height: Option<i64>) -> Picture {
    let original_width = picture.width as i64;
    let original_height = picture.height as i64;
    if original_width <= 0 || original_height <= 0 {
        return picture;
    }
    let width = match width {
        None | Some(0) => original_width,
        Some(-1) => match height {
            Some(height) if height > 0 => original_width * height / original_height,
            _ => original_width,
        },
        Some(width) => width,
    };
    let height = match height {
        None | Some(0) => original_height,
        Some(-1) => original_height * width / original_width,
        Some(height) => height,
    };
    if width <= 0 || height <= 0 {
        return picture;
    }
    let (width, height) = (width as usize, height as usize);
    if width == picture.width && height == picture.height {
        return picture;
    }
    let mut pixels = Vec::with_capacity(width * height * 3);
    for y in 0..height {
        let source_y = y * picture.height / height;
        for x in 0..width {
            let source_x = x * picture.width / width;
            let at = (source_y * picture.width + source_x) * 3;
            if at + 2 >= picture.pixels.len() {
                break;
            }
            pixels.push(picture.pixels[at]);
            pixels.push(picture.pixels[at + 1]);
            pixels.push(picture.pixels[at + 2]);
        }
    }
    Picture {
        pixels,
        width,
        height,
    }
}

/// Look for a picture on the screen.
///
/// The answer is the place of the picture on the screen, and it is
/// nothing when the picture is not there.
pub fn search(
    desktop: &mut LabDesktop,
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64,
    picture: &Picture,
    options: &ImageOptions,
) -> Option<(i64, i64)> {
    let (left, top) = (x1.min(x2), y1.min(y2));
    let (right, bottom) = (x2.max(x1), y2.max(y2));
    let width = right - left + 1;
    let height = bottom - top + 1;
    if width <= 0 || height <= 0 {
        return None;
    }
    let path = desktop
        .capture(left, top, width, height, &format!("ahk-image-{}", millis()))
        .ok()?;
    let screen = read(&path).ok()?;
    let _ = std::fs::remove_file(&path);
    let transparent = options.transparent.as_deref().and_then(parse_colour);
    let variation = options.variation.clamp(0, 255) as i32;
    if picture.width > screen.width || picture.height > screen.height {
        return None;
    }
    for y in 0..=(screen.height - picture.height) {
        for x in 0..=(screen.width - picture.width) {
            if the_picture_is_there(&screen, picture, x, y, variation, transparent) {
                return Some((x as i64 + left, y as i64 + top));
            }
        }
    }
    None
}

/// Whether the picture is at this place of the screen.
fn the_picture_is_there(
    screen: &Picture,
    picture: &Picture,
    x: usize,
    y: usize,
    variation: i32,
    transparent: Option<[u8; 3]>,
) -> bool {
    for row in 0..picture.height {
        for column in 0..picture.width {
            let at = (row * picture.width + column) * 3;
            let there = (y + row) * screen.width + (x + column);
            let at_there = there * 3;
            if at + 2 >= picture.pixels.len() || at_there + 2 >= screen.pixels.len() {
                return false;
            }
            let one = [
                picture.pixels[at],
                picture.pixels[at + 1],
                picture.pixels[at + 2],
            ];
            let other = [
                screen.pixels[at_there],
                screen.pixels[at_there + 1],
                screen.pixels[at_there + 2],
            ];
            // The transparent colour of the picture matches any colour
            // of the screen.
            if let Some(colour) = transparent {
                if one == colour {
                    continue;
                }
            }
            for channel in 0..3 {
                if (one[channel] as i32 - other[channel] as i32).abs() > variation {
                    return false;
                }
            }
        }
    }
    true
}

/// The pictures of the desktop.
impl ahk::desktop::Pictures for LabDesktop {
    fn load_picture(
        &mut self,
        filename: &str,
        options: &PictureOptions,
    ) -> Result<Option<u32>, String> {
        // A file that is not there is not a failure: v2 answers zero.
        if !Path::new(filename).exists() {
            return Ok(None);
        }
        let picture = read(filename)?;
        let picture = resized(picture, options.width, options.height);
        self.pictures.push(picture);
        Ok(Some(self.pictures.len() as u32))
    }

    fn image_search(
        &mut self,
        x1: i64,
        y1: i64,
        x2: i64,
        y2: i64,
        image: &Image,
        options: &ImageOptions,
    ) -> Result<ImageSearch, String> {
        let picture = match image {
            Image::File(name) => {
                if !Path::new(name).exists() {
                    return Ok(ImageSearch::NotLoadable);
                }
                read(name).map_err(|_| String::new())?
            }
            Image::Loaded(handle) => {
                let handle = *handle as usize;
                match self.pictures.get(handle.wrapping_sub(1)) {
                    Some(picture) => Picture {
                        pixels: picture.pixels.clone(),
                        width: picture.width,
                        height: picture.height,
                    },
                    None => return Ok(ImageSearch::NotLoadable),
                }
            }
        };
        let picture = resized(picture, options.width, options.height);
        Ok(match search(self, x1, y1, x2, y2, &picture, options) {
            Some((x, y)) => ImageSearch::Found(x, y),
            None => ImageSearch::NotFound,
        })
    }
}

/// The colour of a transparent pixel, when v2 named one.
///
/// The colour is a number of v2: "#rrggbb", "0xrrggbb" or the name of
/// the colour.
fn parse_colour(colour: &str) -> Option<[u8; 3]> {
    let text = colour.trim();
    let text = text.strip_prefix('#').unwrap_or(text);
    let text = text.strip_prefix("0x").unwrap_or(text);
    if text.len() == 6 {
        let red = u8::from_str_radix(&text[0..2], 16).ok()?;
        let green = u8::from_str_radix(&text[2..4], 16).ok()?;
        let blue = u8::from_str_radix(&text[4..6], 16).ok()?;
        return Some([red, green, blue]);
    }
    match text.to_ascii_lowercase().as_str() {
        "black" => Some([0, 0, 0]),
        "white" => Some([255, 255, 255]),
        "red" => Some([255, 0, 0]),
        "green" => Some([0, 255, 0]),
        "blue" => Some([0, 0, 255]),
        "gray" | "grey" => Some([128, 128, 128]),
        "yellow" => Some([255, 255, 0]),
        "cyan" => Some([0, 255, 255]),
        "magenta" => Some([255, 0, 255]),
        _ => None,
    }
}

/// The milliseconds since the epoch, for the scratch file names.
fn millis() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|moment| moment.as_millis())
        .unwrap_or(0)
}
