extern crate image;
extern crate logo_lib;

use logo_lib::program::Program;
use logo_lib::turtle::Turtle;

fn main() {
    let program = Program::parse("fd 100 rt 45 repeat 4 [rt 90 fd 50]").unwrap();
    let mut turtle = Turtle::new((50.0, 50.0), 0.0, ::image::Rgb([0, 255, 0]));

    let mut image = image::RgbImage::new(500, 500);

    program.run(&mut turtle, &mut image);

    image.save("example_parse.png").unwrap();
}
