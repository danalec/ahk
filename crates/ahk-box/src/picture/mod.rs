//! The pictures of the machine, read here.
//!
//! The box captures the screen in PNGs, and a script asks two things of one:
//! the colour of a pixel, and where a colour is. This module is the half of
//! that answer that belongs to the box -- the file becomes a grid of samples
//! -- and the shape the two questions are asked of.

pub mod crc32;
pub mod png;
pub mod zlib;

pub use png::{Colour, Image, read, write_rgb};