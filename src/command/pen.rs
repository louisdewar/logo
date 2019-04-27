//! Commands to do with the pen (i.e. drawing)

use super::Command;

use parse::{InvalidArgumentError, ParseError, Token};
use turtle::Turtle;
use {Colour, Rgba};

/// Puts the pen up so that no drawing happens
pub struct PenUp;

impl Command for PenUp {
    fn run(&self, turtle: &mut Turtle) {
        turtle.set_pen_down(false);
    }

    fn parse<'a>(
        _tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        Ok(Box::new(PenUp {}))
    }

    fn to_code(&self) -> String {
        "pu".to_string()
    }
}

/// Puts the pen down to start drawing again
pub struct PenDown;

impl Command for PenDown {
    fn run(&self, turtle: &mut Turtle) {
        turtle.set_pen_down(true);
    }

    fn parse<'a>(
        _tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        Ok(Box::new(PenDown {}))
    }

    fn to_code(&self) -> String {
        "pd".to_string()
    }
}

/// Command to turn right by a certain amount
pub struct SetColour {
    colour: Colour,
}

impl SetColour {
    pub fn new(colour: Colour) -> SetColour {
        SetColour { colour }
    }
}

impl Command for SetColour {
    fn run(&self, turtle: &mut Turtle) {
        turtle.set_colour(self.colour)
    }

    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        // Alpha channel defaults to 255
        let mut components = [255_u8; 4];
        for component in components.iter_mut().take(3) {
            // Get next colour component
            let argument = try_word_token!(tokens.next().ok_or(ParseError::NotEnoughArguments)?);

            *component = argument.parse().map_err(|_| {
                InvalidArgumentError {
                    argument: Token::Word(argument),
                    expected: "u8",
                }
                .into()
            })?;
        }

        Ok(Box::new(Self::new(Rgba(components))))
    }

    fn to_code(&self) -> String {
        format!(
            "set_colour {} {} {}",
            self.colour[0], self.colour[1], self.colour[2]
        )
    }
}

/// Clears the screen to black
pub struct Clear {}

impl Clear {}

impl Command for Clear {
    fn run(&self, turtle: &mut Turtle) {
        turtle.set_colour(Rgba([0_u8, 0, 0, 255]));
    }

    fn parse<'a>(
        _tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        Ok(Box::new(Clear {}))
    }

    fn to_code(&self) -> String {
        "clear".to_string()
    }
}

/// Clears the screen to a specified colour
pub struct ClearColour {
    colour: Colour,
}

impl ClearColour {
    pub fn new(colour: Colour) -> ClearColour {
        ClearColour { colour }
    }
}

impl Command for ClearColour {
    fn run(&self, turtle: &mut Turtle) {
        turtle.clear_colour(self.colour)
    }

    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = Token<'a>>,
    ) -> Result<Box<Self>, ParseError<'a>> {
        // Alpha channel defaults to 255
        let mut components = [255_u8; 4];
        for component in components.iter_mut().take(3) {
            // Get next colour component
            let argument = try_word_token!(tokens.next().ok_or(ParseError::NotEnoughArguments)?);

            *component = argument.parse().map_err(|_| {
                InvalidArgumentError {
                    argument: Token::Word(argument),
                    expected: "u8",
                }
                .into()
            })?;
        }

        Ok(Box::new(Self::new(Rgba(components))))
    }

    fn to_code(&self) -> String {
        format!(
            "clear_colour {} {} {}",
            self.colour[0], self.colour[1], self.colour[2]
        )
    }
}
