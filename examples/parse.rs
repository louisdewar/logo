extern crate logo_lib;

use logo_lib::{canvas::Image, Program, Rgba, Turtle};

fn main() {
    let code = r#"
        pu fd 325 rt 90 fd 200 pd rt 45
        repeat 4 [
            set_colour 255 255 255
            repeat 100 [rt 3.6 fd 2]
            set_colour 0 255 0
            repeat 30 [
                pu fd 5 pd rt 90 fd 50 rt 180 fd 100 rt 180 fd 50 rt 270
            ]
            rt 90
        ]
    "#;

    let program = Program::parse(code).unwrap();

    let (draw_queue, mut image) = Image::new(500, 500);

    let mut turtle = Turtle::new((50.0, 50.0), 0.0, Rgba([0, 255, 0, 255]), draw_queue);

    program.run(&mut turtle);

    image.draw_in_queue();
    image
        .get_internal_image()
        .save("example_parse.png")
        .unwrap();
}
