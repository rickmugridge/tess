use std::rc::Rc;
use speedy2d::color::Color;
use crate::frame::Frame;
use crate::painter::{Painter, PainterResponse};
use crate::segment::Segment;
use crate::shape_shifter::Rendering;
use crate::trans_frame::TransFrame;
use crate::transformed_painter::TransformedPainter;

pub struct CompositePainter {
    painters: Vec<Rc<Box<dyn Painter>>>,
}

impl Painter for CompositePainter {
    fn shapes_to_draw(&self, frame: Frame, renderings: &mut Vec<Rendering>) {
        self.painters.iter().for_each(|painter|
            painter.shapes_to_draw(frame.clone(), renderings));
    }

    fn drag_line(&self, line_segment: Segment, thickness: f32, colour: Color) -> Option<PainterResponse> {
        // println!("CompositePainter@20 segment {}", line_segment);
        for painter in &self.painters {
            if let Some(response) = painter.drag_line(line_segment, thickness, colour) {
                // println!("CompositePainter@23 hit {:?}", hit);
                return Some(response);
            }
        }
        // println!("CompositePainter@23 no hit");
        None
    }
}

impl CompositePainter {
    pub fn new(painters: Vec<Rc<Box<dyn Painter>>>) -> Rc<Box<dyn Painter>> {
        Rc::new(Box::new(Self { painters }))
    }

    pub fn beside(painter1: Rc<Box<dyn Painter>>, painter2: Rc<Box<dyn Painter>>) -> Rc<Box<dyn Painter>> {
        let (t_left, t_right) = TransFrame::left_right_halves();
        let left = TransformedPainter::new(painter1, t_left);
        let right = TransformedPainter::new(painter2, t_right);
        Self::new(vec![left, right])
    }

    pub fn directly_beside(painter: Rc<Box<dyn Painter>>, trans_frame: TransFrame) -> Rc<Box<dyn Painter>> {
        Self::beside(painter.clone(), TransformedPainter::new(painter, trans_frame))
    }

    pub fn above(painter1: Rc<Box<dyn Painter>>, painter2: Rc<Box<dyn Painter>>) -> Rc<Box<dyn Painter>> {
        let top = TransformedPainter::new(painter1.clone(),
                                          TransFrame::top_half());
        let bottom = TransformedPainter::new(painter2,
                                             TransFrame::bottom_half());
        Self::new(vec![top, bottom])
    }

    pub fn directly_above(painter: Rc<Box<dyn Painter>>, trans_frame: TransFrame) -> Rc<Box<dyn Painter>> {
        Self::above(painter.clone(), TransformedPainter::new(painter, trans_frame))
    }

    pub fn double_reflect(painter: Rc<Box<dyn Painter>>) -> Rc<Box<dyn Painter>> {
        CompositePainter::directly_beside(
            CompositePainter::directly_above(painter, TransFrame::reflect_y()),
            TransFrame::reflect_x())
    }

    pub fn tessellate_x(painter: Rc<Box<dyn Painter>>, count: usize) -> Rc<Box<dyn Painter>> {
        CompositePainter::tessellate(painter, TransFrame::tessellate_x(count))
    }

    pub fn tessellate_y(painter: Rc<Box<dyn Painter>>, count: usize) -> Rc<Box<dyn Painter>> {
        CompositePainter::tessellate(painter, TransFrame::tessellate_y(count))
    }

    pub fn tessellate_x_y(painter: Rc<Box<dyn Painter>>, x_count: usize, y_count: usize) -> Rc<Box<dyn Painter>> {
        CompositePainter::tessellate_x(
            CompositePainter::tessellate_y(painter, y_count),
            x_count)
    }

    pub fn tessellate(painter: Rc<Box<dyn Painter>>, trans_frames: Vec<TransFrame>) -> Rc<Box<dyn Painter>> {
        let new_painters: Vec<Rc<Box<dyn Painter>>> = trans_frames.iter()
            .map(|tf|
                TransformedPainter::new(painter.clone(), tf.clone())
            )
            .collect();
        Self::new(new_painters)
    }
}
