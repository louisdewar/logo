use crate::command::Command;
use crate::parse::{parse_program, ParseError};
use crate::turtle::Turtle;

/// Represents a set of commands.
/// It is essentially an Abstract Syntax tree
pub struct Program {
    commands: Vec<Box<dyn Command>>,
}

impl Program {
    /// Prase a program object from a string which is the user written program (see language spec)
    pub fn parse(input: &str) -> Result<Program, ParseError> {
        parse_program(input)
    }

    pub fn to_code(&self) -> String {
        use itertools;
        #[allow(clippy::redundant_closure)]
        itertools::join(self.commands.iter().map(|command| command.to_code()), " ")
    }

    /// Create a new program from a set of commands
    pub fn new(commands: Vec<Box<dyn Command>>) -> Program {
        Program { commands }
    }

    pub fn run(&self, turtle: &mut Turtle) {
        for command in &self.commands {
            command.run(turtle);
        }
    }
}
