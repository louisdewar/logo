extern crate image;
extern crate logo_lib;

use logo_lib::command::{basic::*, flow_control::Loop, Command};
use logo_lib::program::Program;
use logo_lib::turtle::Turtle;

fn main() {
    let draw_line = Program::new(vec![
        Box::new(Forward::new(50.0)),
        Box::new(TurnRight::new(::std::f64::consts::FRAC_PI_2)),
    ]);

    let ast: Vec<Box<Command>> = vec![Box::new(Loop::new(draw_line, 4))];

    let program = Program::new(ast);
    let mut turtle = Turtle::new((10.0, 10.0), 0.0, ::image::Rgb([0, 255, 0]));

    let mut image = image::RgbImage::new(500, 500);

    program.run(&mut turtle, &mut image);

    image.save("example_square.png").unwrap();
}
