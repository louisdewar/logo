#[macro_use]
pub mod tokenizer;

pub use self::tokenizer::Token;

use self::tokenizer::{tokenize, ProgramTokens};
use program::Program;

use command::{basic, flow_control, pen, Command};

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError<'a> {
    Syntax(tokenizer::SyntaxError),
    InvalidCommand(&'a str),
    StandaloneProgram(ProgramTokens<'a>),
    UnexpectedProgramArgument(ProgramTokens<'a>),
    InvalidArgument(InvalidArgumentError<'a>),
    // TODO: Deprecate
    InvalidArguments,
    NotEnoughArguments,
}

/// Represents the error of an invalid argument
#[derive(Debug, Clone, PartialEq)]
pub struct InvalidArgumentError<'a> {
    /// The argument that was invalid
    pub argument: Token<'a>,
    /// The human readable message as to what was expected
    pub expected: &'static str,
}

use std::convert::Into;

impl<'a> Into<ParseError<'a>> for InvalidArgumentError<'a> {
    fn into(self) -> ParseError<'a> {
        ParseError::InvalidArgument(self)
    }
}

pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    let tokens_program: ProgramTokens = tokenize(input).map_err(ParseError::Syntax)?;

    parse_program_tokens(tokens_program)
}
pub fn parse_program_tokens(tokens_program: ProgramTokens) -> Result<Program, ParseError> {
    let mut tokens = tokens_program.0.into_iter();

    let mut commands = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::Word(word) => {
                let command: Box<dyn Command> = match word {
                    "fd" => basic::Forward::parse(&mut tokens)?,
                    "rt" => basic::TurnRight::parse(&mut tokens)?,
                    "set_pos" => basic::SetPosition::parse(&mut tokens)?,
                    "repeat" => flow_control::Loop::parse(&mut tokens)?,
                    "pu" => pen::PenUp::parse(&mut tokens)?,
                    "pd" => pen::PenDown::parse(&mut tokens)?,
                    "set_colour" => pen::SetColour::parse(&mut tokens)?,
                    command => return Err(ParseError::InvalidCommand(command)),
                };
                commands.push(command);
            }
            Token::Program(program) => return Err(ParseError::StandaloneProgram(program)),
        }
    }

    Ok(Program::new(commands))
}
