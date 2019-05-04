use crate::canvas::{DrawCommand, Line};
use crate::Colour;
use std::sync::mpsc::Sender;

/// Represents the current drawing state
pub struct Turtle {
    pos: (f64, f64),
    /// Measured in radians, 0 rad means facing east / right
    angle: f64,
    colour: Colour,
    pen_down: bool,
    // A way of sending lines to the draw queue
    draw_queue: Sender<DrawCommand>,
}

impl Turtle {
    pub fn new(
        pos: (f64, f64),
        angle: f64,
        colour: Colour,
        draw_queue: Sender<DrawCommand>,
    ) -> Turtle {
        Turtle {
            pos,
            angle,
            colour,
            pen_down: true,
            draw_queue,
        }
    }

    pub fn set_pos(&mut self, pos: (f64, f64)) {
        self.pos = pos;
    }

    pub fn change_pos(&mut self, delta_pos: &(f64, f64)) {
        self.pos = (self.pos.0 + delta_pos.0, self.pos.1 + delta_pos.1);
    }

    pub fn get_pos(&self) -> &(f64, f64) {
        &self.pos
    }

    pub fn set_angle(&mut self, angle: f64) {
        self.angle = angle;
    }

    pub fn change_angle(&mut self, delta_angle: f64) {
        self.angle += delta_angle;
    }

    pub fn get_angle(&self) -> &f64 {
        &self.angle
    }

    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = colour;
    }

    pub fn get_colour(&self) -> Colour {
        self.colour
    }

    pub fn set_pen_down(&mut self, pen_down: bool) {
        self.pen_down = pen_down
    }

    pub fn is_pen_down(&self) -> bool {
        self.pen_down
    }

    /// Clears the screen to a specified colour
    pub fn clear_colour(&self, colour: Colour) {
        self.draw_queue
            .send(DrawCommand::ClearScreen(colour))
            .expect("Receiver closed channel before drawing was finished")
    }

    /// Sends a line to the draw_queue, this will panic if the receiver of the draw queue has closed its channel
    pub fn draw_line(&self, start: (f64, f64), end: (f64, f64)) {
        let colour = self.get_colour();

        let line = Line { colour, start, end };

        self.draw_queue
            .send(DrawCommand::Line(line))
            .expect("Receiver closed channel before drawing was finished!");
    }
}
