//! The reader of the box, against PNGs of known pixels.
//!
//! Every file under `tests/png/` is written by
//! `egm-review/make_png_fixtures.py`, which builds the chunks, the CRC and
//! the zlib stream by hand: the reader is therefore checked against a writer
//! that is not itself, and a disagreement is a disagreement about the format
//! rather than about one library.
//!
//! The cases are the kinds of pixel the format carries and this reader
//! promises: a colour, a colour and its transparency, a number of grey, a
//! place in a palette, and a grey that is packed a bit to a pixel.

fn read(name: &str) -> ahk_box::picture::Image {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("png")
        .join(name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("the fixture {} is read: {error}", path.display()));
    ahk_box::picture::read(&bytes)
        .unwrap_or_else(|reason| panic!("the fixture {name} is read: {reason}"))
}

#[test]
fn a_colour_is_read_pixel_by_pixel() {
    let image = read("rgb.png");
    assert_eq!((image.width, image.height), (2, 2));
    assert_eq!(image.colour_at(0, 0), Some((255, 0, 0)));
    assert_eq!(image.colour_at(1, 0), Some((0, 255, 0)));
    assert_eq!(image.colour_at(0, 1), Some((0, 0, 255)));
    assert_eq!(image.colour_at(1, 1), Some((255, 255, 255)));
}

#[test]
fn a_colour_with_transparency_answers_its_colour() {
    let image = read("rgba.png");
    assert_eq!(image.colour_at(0, 0), Some((10, 20, 30)));
    assert_eq!(image.colour_at(1, 0), Some((40, 50, 60)));
}

#[test]
fn a_number_of_grey_is_the_three_of_them() {
    let image = read("grey.png");
    assert_eq!(image.colour_at(0, 0), Some((0, 0, 0)));
    assert_eq!(image.colour_at(1, 0), Some((255, 255, 255)));
}

#[test]
fn a_place_in_the_palette_is_the_colour_of_the_place() {
    let image = read("indexed.png");
    assert_eq!(image.colour_at(0, 0), Some((255, 0, 0)));
    assert_eq!(image.colour_at(1, 0), Some((0, 0, 255)));
}

#[test]
fn a_grey_of_one_bit_is_expanded_to_a_byte() {
    let image = read("grey1.png");
    assert_eq!((image.width, image.height), (8, 1));
    // The byte of the row is 0b10101010, from the most significant bit.
    assert_eq!(image.colour_at(0, 0), Some((255, 255, 255)));
    assert_eq!(image.colour_at(1, 0), Some((0, 0, 0)));
    assert_eq!(image.colour_at(2, 0), Some((255, 255, 255)));
    assert_eq!(image.colour_at(3, 0), Some((0, 0, 0)));
    assert_eq!(image.colour_at(6, 0), Some((255, 255, 255)));
    assert_eq!(image.colour_at(7, 0), Some((0, 0, 0)));
}

#[test]
fn a_pixel_that_is_not_in_the_image_has_no_colour() {
    let image = read("rgb.png");
    assert_eq!(image.colour_at(2, 0), None);
    assert_eq!(image.colour_at(0, 2), None);
}

#[test]
fn a_file_that_is_not_a_png_is_refused() {
    let reason = ahk_box::picture::read(b"this is not a picture").unwrap_err();
    assert!(reason.contains("not a PNG"), "{reason}");
}

#[test]
fn a_file_whose_bytes_are_damaged_is_refused() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("png")
        .join("rgb.png");
    let mut bytes = std::fs::read(&path).expect("the fixture is read");
    // A byte of the image data, turned round: the CRC of the chunk no longer
    // matches, and the reader says so rather than reading a wrong colour.
    let at = bytes.len() - 20;
    bytes[at] ^= 0xFF;
    let reason = ahk_box::picture::read(&bytes).unwrap_err();
    assert!(reason.contains("damaged"), "{reason}");
}
/// A capture of the size the compositor writes, to prove the reader walks
/// the rows and not just the first one.
#[test]
fn a_capture_of_two_hundred_pixels_is_read_to_the_last_row() {
    let image = read("big.png");
    assert_eq!((image.width, image.height), (200, 150));
    // The fixture paints every pixel (x mod 256, y mod 256, x*y mod 256).
    assert_eq!(image.colour_at(0, 0), Some((0, 0, 0)));
    assert_eq!(
        image.colour_at(199, 149),
        Some((199, 149, (199u32 * 149u32) as u8))
    );
    assert_eq!(
        image.colour_at(100, 100),
        Some((100, 100, (100u32 * 100u32) as u8))
    );
}
