use image::Rgb;

/// Represents the current drawing state
pub struct Turtle {
    pos: (f64, f64),
    /// Measured in radians, 0 rad means facing east / right
    angle: f64,
    colour: Rgb<u8>,
}

impl Turtle {
    pub fn new(pos: (f64, f64), angle: f64, colour: Rgb<u8>) -> Turtle {
        Turtle { pos, angle, colour }
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

    pub fn set_colour(&mut self, colour: Rgb<u8>) {
        self.colour = colour;
    }

    pub fn get_colour(&self) -> Rgb<u8> {
        self.colour
    }
}
