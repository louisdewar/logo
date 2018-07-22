use std::sync::mpsc::{channel, Receiver, Sender};

use image::RgbaImage;
use Colour;

pub struct Line {
    pub colour: Colour,
    pub start: (f64, f64),
    pub end: (f64, f64),
}

pub struct Image {
    internal_image: RgbaImage,
    receiver: Receiver<Line>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> (Sender<Line>, Image) {
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

    /// Draws as many lines as there are in the queue, it will return when there are no more lines to draw
    pub fn draw_in_queue(&mut self) {
        while let Ok(line) = self.receiver.try_recv() {
            use imageproc::drawing;
            drawing::draw_line_segment_mut(
                &mut self.internal_image,
                (line.start.0.round() as f32, line.start.1.round() as f32),
                (line.end.0.round() as f32, line.end.1.round() as f32),
                line.colour,
            );
        }
    }

    /// Blocks the thread receiving all the lines through the channel until the sender closes it.
    pub fn blocking_receive(&mut self) {
        // Iterate until channel is closed, iterator will block until value becomes available
        for line in self.receiver.iter() {
            use imageproc::drawing;
            drawing::draw_line_segment_mut(
                &mut self.internal_image,
                (line.start.0.round() as f32, line.start.1.round() as f32),
                (line.end.0.round() as f32, line.end.1.round() as f32),
                line.colour,
            );
        }
    }

    pub fn get_internal_image(&self) -> &RgbaImage {
        &self.internal_image
    }
}
