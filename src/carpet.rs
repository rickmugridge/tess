use std::rc::Rc;
use crate::composite_painter::CompositePainter;
use crate::painter::Painter;
use crate::trans_frame::TransFrame;
use crate::transformed_painter::TransformedPainter;
use crate::vector::Vector;

type PainterDetails = (Rc<Box<dyn Painter>>, usize, f32);

pub fn carpet(lefts: Vec<PainterDetails>, // inner to outer
              centre: Rc<Box<dyn Painter>>,
              tops: Vec<PainterDetails>) -> Rc<Box<dyn Painter>> {
    let mut remaining = 2;
    let mut composite = centre.clone();
    let mut lefts_iter = lefts.into_iter();
    let mut tops_iter = tops.into_iter();
    while remaining > 0 {
        if let Some((painter, count, fraction)) = lefts_iter.next() {
            let left_painter = CompositePainter::tessellate_y(painter, count);
            let right_painter = TransformedPainter::new(
                left_painter.clone(), TransFrame::reflect_x());
            composite = CarpetLeftToRight::make(fraction)
                .compose(left_painter, composite, right_painter);
        } else {
            remaining -= 1;
        }
        if let Some((painter, count, fraction)) = tops_iter.next() {
            let top_painter = CompositePainter::tessellate_x(painter, count);
            let bottom_painter = TransformedPainter::new(
                top_painter.clone(), TransFrame::reflect_y());
            composite = CarpetTopToBottom::make(fraction)
                .compose(top_painter, composite, bottom_painter);
        } else {
            remaining -= 1;
        }
    }
    composite
}

pub struct CarpetLeftToRight {
    left: TransFrame,
    centre: TransFrame,
    right: TransFrame,
}

impl CarpetLeftToRight {
    fn new(left: TransFrame,
           centre: TransFrame,
           right: TransFrame) -> Self {
        Self { left, centre, right }
    }

    pub fn make(fraction: f32) -> CarpetLeftToRight {
        assert!(fraction < 0.5);
        let left = TransFrame::new(
            Vector::zero(),
            Vector::new(fraction, 1.0));
        let centre = TransFrame::new(
            Vector::new(fraction, 0.0),
            Vector::new(1.0 - fraction, 1.0));
        let right = TransFrame::new(
            Vector::new(1.0 - fraction, 0.0),
            Vector::new(1.0, 1.0));
        CarpetLeftToRight::new(left, centre, right)
    }

    pub fn compose(&self,
                   left: Rc<Box<dyn Painter>>,
                   centre: Rc<Box<dyn Painter>>,
                   right: Rc<Box<dyn Painter>>) -> Rc<Box<dyn Painter>> {
        CompositePainter::new(vec![
            TransformedPainter::new(left.clone(), self.left.clone()),
            TransformedPainter::new(centre.clone(), self.centre.clone()),
            TransformedPainter::new(right.clone(), self.right.clone())])
    }
}

pub struct CarpetTopToBottom {
    top: TransFrame,
    centre: TransFrame,
    bottom: TransFrame,
}

impl CarpetTopToBottom {
    fn new(top: TransFrame,
           centre: TransFrame,
           bottom: TransFrame) -> Self {
        Self { top, centre, bottom }
    }

    pub fn make(fraction: f32) -> CarpetTopToBottom {
        assert!(fraction < 0.5);
        let top = TransFrame::new(
            Vector::zero(),
            Vector::new(1.0, fraction));
        let centre = TransFrame::new(
            Vector::new(0.0, fraction),
            Vector::new(1.0, 1.0 - fraction));
        let bottom = TransFrame::new(
            Vector::new(0.0, 1.0 - fraction),
            Vector::new(1.0, 1.0));
        CarpetTopToBottom::new(top, centre, bottom)
    }

    pub fn compose(&self,
                   top: Rc<Box<dyn Painter>>,
                   centre: Rc<Box<dyn Painter>>,
                   bottom: Rc<Box<dyn Painter>>) -> Rc<Box<dyn Painter>> {
        CompositePainter::new(vec![
            TransformedPainter::new(top.clone(), self.top.clone()),
            TransformedPainter::new(centre.clone(), self.centre.clone()),
            TransformedPainter::new(bottom.clone(), self.bottom.clone())])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn left_to_right() {
        let carpet = CarpetLeftToRight::make(0.1);
        assert_eq!(carpet.left, TransFrame::new(Vector::new(0.0, 0.0),
                                                Vector::new(0.1, 1.0)));
        assert_eq!(carpet.centre, TransFrame::new(Vector::new(0.1, 0.0),
                                                  Vector::new(0.9, 1.0)));
        assert_eq!(carpet.right, TransFrame::new(Vector::new(0.9, 0.0),
                                                 Vector::new(1.0, 1.0)));
    }

    #[test]
    fn top_to_bottom() {
        let carpet = CarpetTopToBottom::make(0.1);
        assert_eq!(carpet.top, TransFrame::new(Vector::new(0.0, 0.0),
                                               Vector::new(1.0, 0.1)));
        assert_eq!(carpet.centre, TransFrame::new(Vector::new(0.0, 0.1),
                                                  Vector::new(1.0, 0.9)));
        assert_eq!(carpet.bottom, TransFrame::new(Vector::new(0.0, 0.9),
                                                  Vector::new(1.0, 1.0)));
    }
}