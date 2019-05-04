#![deny(clippy::all)]

extern crate colored;
extern crate image;
extern crate imageproc;
extern crate itertools;

#[macro_use]
pub mod parse;

pub mod canvas;
pub mod command;
pub mod program;
pub mod turtle;

#[cfg(test)]
mod tests;

pub use image::{RgbImage, Rgba};
pub use crate::program::Program;
pub use crate::turtle::Turtle;

pub type Colour = image::Rgba<u8>;
