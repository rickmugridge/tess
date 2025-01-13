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

pub struct ThicknessSelectorPainter {
    shape_index: usize,
    thickness: f32,
}

impl Painter for ThicknessSelectorPainter {
    fn shapes_to_draw(&self, frame: Frame, renderings: &mut Vec<Rendering>) {
        renderings.push(Rendering::new(self.shape_index, frame));
    }

    fn drag_line(&self, line_segment: Segment, _thickness: f32, _colour: Color) -> Option<PainterResponse> {
        if line_segment.inside_unit() {
            Some(PainterResponse::ThicknessChange(self.thickness))
        } else { None }
    }
}

impl ThicknessSelectorPainter {
    pub fn new(shape_index: usize, thickness: f32) -> Rc<Box<dyn Painter>> {
        Rc::new(Box::new(Self { shape_index, thickness }))
    }

    pub fn make_panel(shapes: Vec<(usize, f32)>) -> Rc<Box<dyn Painter>> {
        let frames = TransFrame::tessellate_y(shapes.len());
        let painters = shapes.into_iter()
            .zip(frames)
            .map(|((shape_index, thickness), trans_frame)|
                Self::make_thickness(shape_index, thickness, trans_frame));
        CompositePainter::new(painters.collect())
    }

    fn make_thickness(shape_index: usize, thickness: f32, trans_frame: TransFrame) -> Rc<Box<dyn Painter>> {
        let thick = Self::new(shape_index, thickness);
        let thick = TransformedPainter::bordered(
            thick, Vector::new(0.0, 0.2), Vector::new(1.0, 0.8));
        let thick = TransformedPainter::new(thick, trans_frame);
        thick
    }
}