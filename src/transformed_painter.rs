use std::rc::Rc;
use speedy2d::color::Color;
use crate::frame::Frame;
use crate::painter::{Painter, PainterResponse};
use crate::segment::Segment;
use crate::shape_shifter::Rendering;
use crate::trans_frame::TransFrame;
use crate::vector::Vector;

pub struct TransformedPainter {
    painter: Rc<Box<dyn Painter>>,
    trans_frame: TransFrame,
}

impl Painter for TransformedPainter {
    fn shapes_to_draw(&self, frame: Frame, renderings: &mut Vec<Rendering>) {
        let new_frame = frame.transform_by_frame(&self.trans_frame);
        self.painter.shapes_to_draw(new_frame, renderings);
    }

    fn drag_line(&self, line_segment: Segment, thickness: f32, colour: Color) -> Option<PainterResponse> {
        // println!("TransformedPainter@20 segment {}", line_segment);
        let segment = line_segment.reverse_translate(&self.trans_frame);
        // println!("TransformedPainter@22 translated segment {:?}", segment);
        self.painter.drag_line(segment, thickness, colour)
    }
}

impl TransformedPainter {
    pub fn new(painter: Rc<Box<dyn Painter>>, trans_frame: TransFrame) -> Rc<Box<dyn Painter>> {
        Rc::new(Box::new(Self { painter, trans_frame }))
    }

    pub fn bordered(painter: Rc<Box<dyn Painter>>,
                    origin: Vector, corner: Vector) -> Rc<Box<dyn Painter>> {
        Self::new(painter, TransFrame::new(origin, corner))
    }
}