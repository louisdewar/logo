use image::RgbImage;
use turtle::Turtle;

pub mod basic;
pub mod flow_control;

/// Represents a command
pub trait Command {
    /// Run the command possibly changing the turtle or drawing to the output image
    fn run(&self, turtle: &mut Turtle, image: &mut RgbImage);

    /// Parse the command, gets given an iterator which represents the stream of tokens
    /// The parser should only `take()` the number of tokens it needs, since the stream will contain tokens of other commands
    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = &'a str>,
    ) -> Result<Box<Self>, ::program::ParseError>
    where
        Self: Sized;

    /// Convert the `Command` to the logo code representation of it
    fn to_code(&self) -> String;
}
