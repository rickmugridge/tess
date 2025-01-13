use std::fmt::{Display, Formatter};
use crate::frame::Frame;
use crate::trans_frame::TransFrame;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Segment {
    pub start: Vector,
    pub end: Vector,
}

impl Segment {
    pub fn new(start: Vector, end: Vector) -> Self {
        Segment { start, end }
    }

    pub fn new_from_pairs(start: (f32, f32), end: (f32, f32)) -> Self {
        Segment::new(Vector::new_from_pair(start),
                     Vector::new_from_pair(end))
    }

    pub fn open_path(v: &[Vector]) -> Vec<Segment> {
        assert!(v.len() > 0);
        let mut previous = &v[0];
        v.iter().skip(1).map(|v| {
            let segment = Segment::new(*previous, *v);
            previous = v;
            segment
        }).collect()
    }

    pub fn translate(&self, plus: &Vector) -> Self {
        Segment::new(self.start + *plus, self.end + *plus)
    }

    pub fn reverse_translate(&self, trans: &TransFrame) -> Self {
        Segment::new(trans.reverse_translate(&self.start),
                     trans.reverse_translate(&self.end))
    }

    pub fn translate_to(&self, frame: &Frame) -> Self {
        Segment::new(frame.translate(&self.start), frame.translate(&self.end))
    }

    pub fn scale2d(&self, offset: &Vector) -> Segment {
        Segment::new(self.start.scale2d(offset), self.end.scale2d(offset))
    }

    pub fn inside_unit(&self) -> bool {
        self.start.inside_unit() && self.end.inside_unit()
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "segment({} -> {})", self.start, self.end)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_path() {
        let v1 = Vector::new(1.0, 2.0);
        let v2 = Vector::new(3.0, 4.0);
        let v3 = Vector::new(4.0, 6.0);
        let path = vec![v1, v2, v3, v1];
        assert_eq!(Segment::open_path(&path), vec![
            Segment::new(v1, v2),
            Segment::new(v2, v3),
            Segment::new(v3, v1),
        ]);
    }

    #[test]
    fn transform() {
        let v3 = Vector::new(4.0, 6.0);
        let segment = Segment::new_from_pairs(
            (1.0, 2.0),
            (3.0, 4.0));
        assert_eq!(segment.translate(&v3),
                   Segment::new_from_pairs((5.0, 8.0),
                                           (7.0, 10.0)));
    }

    #[test]
    fn scale2d() {
        let offset = Vector::new(5.0, 6.0);
        let segment = Segment::new_from_pairs(
            (0.1, 0.2),
            (0.4, 0.5));
        assert_eq!(segment.scale2d(&offset),
                   Segment::new_from_pairs((0.5, 1.2),
                                           (2.0, 3.0)));
    }
}