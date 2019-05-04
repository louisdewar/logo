//! All the basic commands that operate the turtle (focused on movement)

use super::Command;

use crate::parse::{ParseError, Token};
use crate::turtle::Turtle;

/// Command to move forward by a certain amount
pub struct Forward {
    amount: f64,
}

impl Forward {
    pub fn new(amount: f64) -> Forward {
        Forward { amount }
    }
}

impl Command for Forward {
    fn run(&self, turtle: &mut Turtle) {
        // TODO: Wait for NLL fix to remove unnecessary scope
        let (start, end) = {
            let angle = turtle.get_angle();
            let delta = ((self.amount * angle.cos()), (self.amount * angle.sin()));

            let start = turtle.get_pos();
            let end = ((start.0 + delta.0), (start.1 + delta.1));

            (*start, end)
        };

        // If pen is down then draw
        if turtle.is_pen_down() {
            turtle.draw_line(start, end);
        }

        turtle.set_pos(end);
    }

    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        let amount = try_word_token!(tokens.next().ok_or(ParseError::NotEnoughArguments)?)
            .parse()
            .map_err(|_| ParseError::InvalidArguments)?;

        Ok(Box::new(Self::new(amount)))
    }

    fn to_code(&self) -> String {
        format!("fd {}", self.amount)
    }
}

/// Command to turn right by a certain amount
pub struct TurnRight {
    amount: f64,
}

impl TurnRight {
    pub fn new(amount: f64) -> TurnRight {
        TurnRight { amount }
    }
}

impl Command for TurnRight {
    fn run(&self, turtle: &mut Turtle) {
        turtle.change_angle(self.amount);
    }

    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        // Angle in degrees
        let amount = try_word_token!(tokens.next().ok_or(ParseError::NotEnoughArguments)?)
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidArguments)?;

        Ok(Box::new(Self::new(amount.to_radians())))
    }

    fn to_code(&self) -> String {
        // TODO: Decide if this is the right way to go forward
        format!("rt {}", self.amount.to_degrees().round())
    }
}

/// Command to move forward by a certain amount
pub struct SetPosition {
    x: f64,
    y: f64,
}

impl SetPosition {
    pub fn new(x: f64, y: f64) -> SetPosition {
        SetPosition { x, y }
    }
}

impl Command for SetPosition {
    fn run(&self, turtle: &mut Turtle) {
        turtle.set_pos((self.x, self.y));
    }

    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        let x = try_word_token!(tokens.next().ok_or(ParseError::NotEnoughArguments)?)
            .parse()
            .map_err(|_| ParseError::InvalidArguments)?;

        let y = try_word_token!(tokens.next().ok_or(ParseError::NotEnoughArguments)?)
            .parse()
            .map_err(|_| ParseError::InvalidArguments)?;

        Ok(Box::new(Self::new(x, y)))
    }

    fn to_code(&self) -> String {
        format!("set_pos {} {}", self.x, self.y)
    }
}
