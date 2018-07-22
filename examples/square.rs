extern crate logo_lib;

use logo_lib::command::{basic::*, flow_control::Loop, Command};
use logo_lib::{canvas::Image, Program, Rgba, Turtle};

fn main() {
    let draw_line = Program::new(vec![
        Box::new(Forward::new(50.0)),
        Box::new(TurnRight::new(::std::f64::consts::FRAC_PI_2)),
    ]);

    let ast: Vec<Box<Command>> = vec![Box::new(Loop::new(draw_line, 4))];

    let program = Program::new(ast);

    let (draw_queue, mut image) = Image::new(500, 500);

    let mut turtle = Turtle::new((50.0, 50.0), 0.0, Rgba([0, 255, 0, 255]), draw_queue);

    program.run(&mut turtle);

    image.draw_in_queue();
    image
        .get_internal_image()
        .save("example_square.png")
        .unwrap();
}
