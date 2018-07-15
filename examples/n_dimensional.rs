extern crate image;
extern crate logo_lib;

use logo_lib::command::{basic::*, flow_control::Loop, Command};
use logo_lib::program::Program;
use logo_lib::turtle::Turtle;

const N: u32 = 100;
const LENGTH: f64 = 5.0;

fn main() {
    let draw_line = Program::new(vec![
        Box::new(Forward::new(LENGTH)),
        Box::new(TurnRight::new(2.0 * ::std::f64::consts::PI / N as f64)),
    ]);

    let ast: Vec<Box<Command>> = vec![Box::new(Loop::new(draw_line, N))];

    let program = Program::new(ast);
    let mut turtle = Turtle::new(
        (50.0, 50.0),
        -::std::f64::consts::FRAC_2_PI,
        ::image::Rgb([0, 255, 0]),
    );

    let mut image = image::RgbImage::new(500, 500);

    program.run(&mut turtle, &mut image);

    image.save("example_n_dimensional.png").unwrap();
}
