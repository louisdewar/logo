use command::Command;
use image::RgbImage;
use turtle::Turtle;

/// Represents a set of commands.
/// It is essentially an Abstract Syntax tree
pub struct Program {
    commands: Vec<Box<dyn Command>>,
}

impl Program {
    /// Prase a program object from a string which is the user written program (see language spec)
    pub fn parse(input: &str) -> Result<Program, ParseError> {
        let mut stream = input.split(' ');

        Self::parse_from_tokens(&mut stream)
    }

    pub fn parse_from_tokens<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = &'a str>,
    ) -> Result<Program, ParseError> {
        use command::*;
        let mut commands: Vec<Box<dyn Command>> = vec![];

        while let Some(token) = tokens.next() {
            match token {
                "fd" => commands.push(basic::Forward::parse(tokens)?),
                "rt" => commands.push(basic::TurnRight::parse(tokens)?),
                "repeat" => commands.push(flow_control::Loop::parse(tokens)?),
                _ => return Err(ParseError::InvalidCommand),
            };
        }

        Ok(Program::new(commands))
    }

    pub fn to_code(&self) -> String {
        use itertools;
        itertools::join(self.commands.iter().map(|command| command.to_code()), " ")
    }

    /// Create a new program from a set of commands
    pub fn new(commands: Vec<Box<dyn Command>>) -> Program {
        Program { commands }
    }

    pub fn run(&self, turtle: &mut Turtle, canvas: &mut RgbImage) {
        for command in &self.commands {
            command.run(turtle, canvas);
        }
    }
}

#[derive(Clone, Debug)]
pub enum ParseError {
    InvalidArguments,
    InvalidCommand,
    UnexpectedToken,
    NotEnoughArguments,
    MissingClosingParenthesis,
}
