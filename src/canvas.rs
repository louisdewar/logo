use std::sync::mpsc::{channel, Receiver, Sender};

use image::{ImageBuffer, RgbaImage};
use crate::Colour;

pub struct Line {
    pub colour: Colour,
    pub start: (f64, f64),
    pub end: (f64, f64),
}

pub enum DrawCommand {
    Line(Line),
    ClearScreen(Colour),
}

pub struct Image {
    internal_image: RgbaImage,
    receiver: Receiver<DrawCommand>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> (Sender<DrawCommand>, Image) {
        let internal_image = RgbaImage::new(width, height);

        let (sender, receiver) = channel();

        (
            sender,
            Image {
                internal_image,
                receiver,
            },
        )
    }

    fn handle_command(image: &mut RgbaImage, draw_command: DrawCommand) {
        match draw_command {
            DrawCommand::Line(line) => {
                use imageproc::drawing;
                drawing::draw_line_segment_mut(
                    image,
                    (line.start.0.round() as f32, line.start.1.round() as f32),
                    (line.end.0.round() as f32, line.end.1.round() as f32),
                    line.colour,
                );
            }
            DrawCommand::ClearScreen(colour) => {
                let (width, height) = image.dimensions();

                *image = ImageBuffer::from_pixel(width, height, colour);
            }
        }
    }

    /// Draws as many lines as there are in the queue, it will return when there are no more lines to draw
    pub fn draw_in_queue(&mut self) {
        while let Ok(command) = self.receiver.try_recv() {
            Self::handle_command(&mut self.internal_image, command);
        }
    }

    /// Blocks the thread receiving all the lines through the channel until the sender closes it.
    pub fn blocking_receive(&mut self) {
        // Iterate until channel is closed, iterator will block until value becomes available
        for command in self.receiver.iter() {
            Self::handle_command(&mut self.internal_image, command);
        }
    }

    pub fn get_internal_image(&self) -> &RgbaImage {
        &self.internal_image
    }
}
