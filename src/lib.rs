#![deny(clippy::all)]


use image;

use itertools;

#[macro_use]
pub mod parse;

pub mod canvas;
pub mod command;
pub mod program;
pub mod turtle;

#[cfg(test)]
mod tests;

pub use crate::program::Program;
pub use crate::turtle::Turtle;
pub use image::{RgbImage, Rgba};

pub type Colour = image::Rgba<u8>;
