//! Commands to do with flow control, such as Loop

use super::Command;
use crate::parse::{parse_program_tokens, ParseError, Token};
use crate::program::Program;
use crate::turtle::Turtle;

/// A loop will run a given program for a certain specified number of times
pub struct Loop {
    program: Program,
    times: u32,
}

impl Loop {
    /// Takes the program to run and the number of times to run it
    pub fn new(program: Program, times: u32) -> Loop {
        Loop { program, times }
    }
}

impl Command for Loop {
    fn run(&self, turtle: &mut Turtle) {
        for _ in 0..self.times {
            self.program.run(turtle);
        }
    }

    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        // Get the number of times to loop
        let times = try_word_token!(tokens.next().ok_or(ParseError::NotEnoughArguments)?)
            .parse()
            .map_err(|_| ParseError::InvalidArguments)?;

        let program = match tokens.next().ok_or(ParseError::NotEnoughArguments)? {
            Token::Word(_) => return Err(ParseError::InvalidArguments),
            Token::Program(program) => parse_program_tokens(program)?,
        };

        Ok(Box::new(Self::new(program, times)))
    }

    fn to_code(&self) -> String {
        format!("repeat {} [{}]", self.times, self.program.to_code())
    }
}
