use std::rc::Rc;
use speedy2d::color::Color;
use crate::composite_painter::CompositePainter;
use crate::frame::Frame;
use crate::painter::{Painter, PainterResponse};
use crate::segment::Segment;
use crate::shape_shifter::Rendering;
use crate::trans_frame::TransFrame;
use crate::transformed_painter::TransformedPainter;
use crate::vector::Vector;

pub struct ColourSelectorPainter {
    shape_index: usize,
    colour: Color,
}

impl Painter for ColourSelectorPainter {
    fn shapes_to_draw(&self, frame: Frame, renderings: &mut Vec<Rendering>) {
        renderings.push(Rendering::new(self.shape_index, frame));
    }

    fn drag_line(&self, line_segment: Segment, _thickness: f32, _colour: Color) -> Option<PainterResponse> {
        if line_segment.inside_unit() {
            Some(PainterResponse::ColorChange(self.colour))
        } else { None }
    }
}

impl ColourSelectorPainter {
    pub fn new(shape_index: usize, colour: Color) -> Rc<Box<dyn Painter>> {
        Rc::new(Box::new(Self { shape_index, colour }))
    }

    pub fn make_panel(shapes: Vec<(usize, Color)>) -> Rc<Box<dyn Painter>> {
        let frames = TransFrame::tessellate_y(shapes.len());
        let painters = shapes.into_iter()
            .zip(frames)
            .map(|((shape_index, colour), trans_frame)|
                Self::make_colour(shape_index, colour, trans_frame));
        CompositePainter::new(painters.collect())
    }

    fn make_colour(shape_index: usize, colour: Color, trans_frame: TransFrame) -> Rc<Box<dyn Painter>> {
        let blue = Self::new(shape_index, colour);
        let blue = TransformedPainter::bordered(
            blue, Vector::new(0.0, 0.2), Vector::new(1.0, 0.8));
        let blue = TransformedPainter::new(blue, trans_frame);
        blue
    }
}