extern crate image;
extern crate logo_lib;

use logo_lib::command::basic::Forward;
use logo_lib::program::Program;
use logo_lib::turtle::Turtle;

fn main() {
    let program = Program::new(vec![Box::new(Forward::new(50.0))]);
    let mut turtle = Turtle::new(
        (0.0, 0.0),
        ::std::f64::consts::FRAC_PI_4 + 0.2,
        ::image::Rgb([0, 255, 0]),
    );

    let mut image = image::RgbImage::new(500, 500);

    program.run(&mut turtle, &mut image);

    image.save("example_simple.png").unwrap();
}
