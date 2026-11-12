//! The pictures: the images of the machine.
//!
//! v2 has two names for them: `LoadPicture` loads a picture of a file
//! and answers the number that identifies it, and `ImageSearch` looks
//! for a picture on the screen. The options of each one are read here,
//! because they are the dialect's, and the image is the machine's.
//!
//! Measured on the binary of v2.0.19 on 2026-10-29.

use alloc::string::{String, ToString};

use crate::desktop::{Image, ImageOptions, ImageSearch, PictureOptions};
use crate::runtime::{Runtime, RuntimeError, Value};

/// `LoadPicture`: load a picture of a file.
///
/// The answer of v2 is a number that identifies the picture, and it is
/// zero when the picture could not be loaded: a file that is not there
/// is not a failure.
///
/// Measured on the binary of v2.0.19 on 2026-10-29: the answer is zero
/// when the picture is not there, the type of the handle is written in
/// the third parameter, and an option that v2 does not know is ignored.
pub fn load_picture(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    let filename = text_of(values, 0);
    let options = the_options_of_the_picture(&text_of(values, 1));
    let handle = rt
        .desktop()
        .load_picture(&filename, &options)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    // The type of the handle is written in the third parameter, when
    // there is one: it is zero, the bitmap of v2.
    if let Some(value) = values.get(2) {
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(3, "LoadPicture", "a VarRef", value),
                line,
            });
        }
        rt.write_reference(value, Value::Int(0));
    }
    Ok(Value::Int(handle.unwrap_or(0) as i64))
}

/// `ImageSearch`: look for a picture on the screen.
///
/// The answer of v2 is one when the picture is there and zero when it
/// is not, and the place of the picture is written in the first two
/// variables of the caller.
///
/// Measured on the binary of v2.0.19 on 2026-10-29: the first two
/// parameters want a variable, and a picture that cannot be loaded is
/// refused with "Parameter #7 of ImageSearch is invalid.".
pub fn image_search(
    rt: &mut Runtime<'_>,
    values: &[Value],
    line: u32,
    _variant: u8,
) -> Result<Value, RuntimeError> {
    // The place of the picture is written in the first two parameters,
    // and they are given by reference.
    for index in 0..2 {
        let Some(value) = values.get(index) else {
            continue;
        };
        if !matches!(value, Value::Reference(_)) {
            return Err(RuntimeError {
                message: crate::runtime::wants_a_reference(
                    index + 1,
                    "ImageSearch",
                    "a VarRef",
                    value,
                ),
                line,
            });
        }
    }
    let x1 = number_of(values, 2) as i64;
    let y1 = number_of(values, 3) as i64;
    let x2 = number_of(values, 4) as i64;
    let y2 = number_of(values, 5) as i64;
    let (image, options) = the_picture_of_the_search(&text_of(values, 6));
    let answer = rt
        .desktop()
        .image_search(x1, y1, x2, y2, &image, &options)
        .map_err(|reason| RuntimeError {
            message: reason,
            line,
        })?;
    match answer {
        ImageSearch::Found(x, y) => {
            rt.write_reference(&values[0], Value::Int(x));
            rt.write_reference(&values[1], Value::Int(y));
            Ok(Value::Int(1))
        }
        ImageSearch::NotFound => {
            // The place of v2 is empty when the picture is not there.
            rt.write_reference(&values[0], Value::Text(String::new()));
            rt.write_reference(&values[1], Value::Text(String::new()));
            Ok(Value::Int(0))
        }
        ImageSearch::NotLoadable => Err(RuntimeError {
            message: crate::runtime::invalid_parameter(7, "ImageSearch"),
            line,
        }),
    }
}

// -- the options ----------------------------------------------------------------

/// The options of a picture.
///
/// The options are the ones of v2: "Wn" and "Hn" are the size the
/// picture is loaded at, "Iconn" is the icon of a file that has many,
/// and "GDI+" is a name of Windows. An option that v2 does not know
/// is ignored, and the tree ignores it too.
fn the_options_of_the_picture(options: &str) -> PictureOptions {
    let mut read = PictureOptions::default();
    for token in options.split([' ', '\t']).filter(|token| !token.is_empty()) {
        let mut letters = token.chars();
        let letter = letters.next().unwrap_or_default().to_ascii_lowercase();
        let rest = letters.as_str();
        match letter {
            'w' => read.width = the_number_of_an_option(rest),
            'h' => read.height = the_number_of_an_option(rest),
            'i' => read.icon = the_number_of_an_option(rest),
            _ => {}
        }
    }
    read
}

/// The picture of a search, and the options of the search.
///
/// v2 writes the options of the search in front of the name of the
/// picture, and every option of them begins with a star: "*20
/// *TransBlack C:\\Main Logo.png". The name of the picture is what is
/// left of it.
fn the_picture_of_the_search(text: &str) -> (Image, ImageOptions) {
    let mut options = ImageOptions::default();
    let mut rest = text.trim();
    while let Some(after) = rest.strip_prefix('*') {
        let (token, tail) = match after.find(' ') {
            Some(at) => (&after[..at], after[at..].trim_start()),
            None => (after, ""),
        };
        let mut letters = token.chars();
        let letter = letters.next().unwrap_or_default().to_ascii_lowercase();
        let number = letters.as_str();
        match letter {
            't' => {
                // The transparent colour: the rest of the option is the
                // colour, and it may be a name.
                let colour = if number.to_ascii_lowercase().starts_with("rans") {
                    &number[5..]
                } else {
                    number
                };
                options.transparent = Some(colour.to_string());
            }
            'w' => options.width = the_number_of_an_option(number),
            'h' => options.height = the_number_of_an_option(number),
            'i' => {}
            // The variation is a number alone, and it is the option that
            // has no letter.
            _ => options.variation = the_number_of_an_option(number).unwrap_or(0),
        }
        if tail.is_empty() {
            rest = "";
            break;
        }
        rest = tail;
    }
    (the_image_of_the_search(rest), options)
}

/// The picture of a search: the name of a file, or a picture that was
/// loaded before.
///
/// v2 writes the handle of a picture that was loaded before in the
/// spelling of Windows: "HBITMAP:*123".
fn the_image_of_the_search(text: &str) -> Image {
    let text = text.trim();
    for prefix in ["HBITMAP:*", "HBITMAP:", "BITMAP:*"] {
        if let Some(rest) = text.strip_prefix(prefix) {
            if let Ok(handle) = rest.trim().parse::<u32>() {
                return Image::Loaded(handle);
            }
        }
    }
    Image::File(text.to_string())
}

/// The number that follows a letter of the options.
fn the_number_of_an_option(text: &str) -> Option<i64> {
    if text.is_empty() {
        return None;
    }
    text.parse::<f64>().ok().map(|number| number as i64)
}

// -- the helpers ----------------------------------------------------------------

/// The text of the argument at this place, or the empty string.
fn text_of(values: &[Value], index: usize) -> String {
    values
        .get(index)
        .map(|value| value.to_text())
        .unwrap_or_default()
}

/// The number of the argument at this place, or zero.
fn number_of(values: &[Value], index: usize) -> f64 {
    values
        .get(index)
        .and_then(|value| value.to_number())
        .unwrap_or(0.0)
}
