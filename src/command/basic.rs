//! All the basic commands that operate the turtle (focused on movement)

use super::Command;

use image::RgbImage;
use turtle::Turtle;

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
    fn run(&self, turtle: &mut Turtle, image: &mut RgbImage) {
        // TODO: Wait for NLL fix to remove unnecessary scope
        let end = {
            let angle = turtle.get_angle();
            let delta = ((self.amount * angle.cos()), (self.amount * angle.sin()));

            let (start_x, start_y) = turtle.get_pos();
            let (end_x, end_y) = ((start_x + delta.0), (start_y + delta.1));

            use imageproc::drawing;
            drawing::draw_line_segment_mut(
                image,
                (start_x.round() as f32, start_y.round() as f32),
                (end_x.round() as f32, end_y.round() as f32),
                turtle.get_colour(),
            );

            (end_x, end_y)
        };

        turtle.set_pos(end);
    }

    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = &'a str>,
    ) -> Result<Box<Self>, ::program::ParseError> {
        let amount = tokens
            .next()
            .ok_or(::program::ParseError::NotEnoughArguments)?
            .parse()
            .map_err(|_| ::program::ParseError::InvalidArguments)?;

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
    fn run(&self, turtle: &mut Turtle, _: &mut RgbImage) {
        turtle.change_angle(self.amount);
    }

    fn parse<'a>(
        tokens: &mut impl ::std::iter::Iterator<Item = &'a str>,
    ) -> Result<Box<Self>, ::program::ParseError> {
        // Angle in degrees
        let amount = tokens
            .next()
            .ok_or(::program::ParseError::NotEnoughArguments)?
            .parse::<f64>()
            .map_err(|_| ::program::ParseError::InvalidArguments)?;

        Ok(Box::new(Self::new(amount.to_radians())))
    }

    fn to_code(&self) -> String {
        // TODO: Decide if this is the right way to go forward
        format!("rt {}", self.amount.to_degrees().round())
    }
}
