use parse::{ParseError, Token};
use turtle::Turtle;

pub mod basic;
pub mod flow_control;
pub mod pen;

/// Represents a command
pub trait Command {
    /// Run the command
    fn run(&self, turtle: &mut Turtle);

    /// Parse the command, gets given an iterator which represents the stream of tokens
    /// The parser should only `take()` the number of tokens it needs, since the stream will contain tokens of other commands
    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>>
    where
        Self: Sized;

    /// Convert the `Command` to the logo code representation of it
    fn to_code(&self) -> String;
}
