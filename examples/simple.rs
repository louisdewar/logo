use logo_lib::command::basic::Forward;
use logo_lib::{canvas::Image, Program, Rgba, Turtle};

fn main() {
    let program = Program::new(vec![Box::new(Forward::new(50.0))]);

    let (draw_queue, mut image) = Image::new(500, 500);

    let mut turtle = Turtle::new(
        (50.0, 50.0),
        ::std::f64::consts::FRAC_PI_4 + 0.2,
        Rgba([0, 255, 0, 255]),
        draw_queue,
    );

    program.run(&mut turtle);

    image.draw_in_queue();
    image
        .get_internal_image()
        .save("example_simple.png")
        .unwrap();
}
