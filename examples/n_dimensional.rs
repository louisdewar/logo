

use logo_lib::command::{basic::*, flow_control::Loop, pen::*, Command};
use logo_lib::{canvas::Image, Program, Rgba, Turtle};

use std::f64::consts;

// The radius of the shape, this is the distance from a corner to the center.
// This is set to the width of the (image / 2) - 50 to give it a bit of padding
const RADIUS: f64 = (500.0 / 2.0) - 50.0;

fn main() {
    // Setup
    let (draw_queue, mut image) = Image::new(500, 500);

    let mut turtle = Turtle::new(
        (0.0, 0.0),
        -consts::FRAC_PI_2,
        Rgba([0, 255, 0, 255]),
        draw_queue,
    );

    // Draw shapes with number of sides n
    for n in 3..100 {
        // The length of each side is calculated so that it will create the desired radius
        // (r * sin(360 / n)) / (cos(180 / n))
        let length =
            (RADIUS * (2.0 * consts::PI / f64::from(n)).sin()) / (consts::PI / f64::from(n)).cos();

        let draw_line = Program::new(vec![
            Box::new(TurnRight::new(2.0 * consts::PI / f64::from(n))),
            Box::new(Forward::new(length)),
        ]);

        let ast: Vec<Box<dyn Command>> = vec![
            // Set position to be centre
            Box::new(SetPosition::new(250.0, 250.0)),
            Box::new(PenUp {}),
            // Move from center to edge of shape
            Box::new(Forward::new(RADIUS)),
            // Turn right so that it is on correctly on the edge of the shape
            Box::new(TurnRight::new(
                consts::FRAC_PI_2 - (consts::PI / f64::from(n)),
            )),
            Box::new(PenDown {}),
            // Loop and draw shape
            Box::new(Loop::new(draw_line, n)),
        ];

        let program = Program::new(ast);

        // Run the program for each shape
        program.run(&mut turtle);
    }

    image.draw_in_queue();
    image
        .get_internal_image()
        .save("example_n_dimensional.png")
        .unwrap();
}
