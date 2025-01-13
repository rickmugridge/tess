use std::rc::Rc;
use speedy2d::color::Color;
use crate::frame::Frame;
use crate::segment::Segment;
use crate::shape::Shape;
use crate::shape_shifter::{AddShapeDetails, Rendering};

pub trait Painter {
    fn shapes_to_draw(&self, frame: Frame, renderings: &mut Vec<Rendering>);
    fn drag_line(&self, line_segment: Segment<>,
                 thickness: f32, colour: Color) -> Option<PainterResponse>;
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnitPainter {
    shape_index: usize,
}

impl Painter for UnitPainter {
    fn shapes_to_draw(&self, frame: Frame, renderings: &mut Vec<Rendering>) {
        renderings.push(Rendering::new(self.shape_index, frame));
    }

    fn drag_line(&self, line_segment: Segment, thickness: f32, colour: Color) -> Option<PainterResponse> {
        // println!("UnitPainter@24 segment {} inside: {}", line_segment, line_segment.inside_unit());
        if line_segment.inside_unit() {
            let line = Shape::Line(line_segment, thickness, colour);
            Some(PainterResponse::Add(AddShapeDetails::new(self.shape_index, line)))
        } else { None }
    }
}

impl UnitPainter {
    pub fn new(shape_index: usize) -> Rc<Box<dyn Painter>> {
        Rc::new(Box::new(Self { shape_index }))
    }
}

pub enum PainterResponse {
    Add(AddShapeDetails),
    ColorChange(Color),
    ThicknessChange(f32),
}