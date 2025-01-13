use speedy2d::dimen::Vector2;
use crate::shape::Shape;
use crate::trans_frame::TransFrame;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct Frame {
    pub origin: Vector,
    pub offset: Vector,
}

impl Frame {
    pub fn new(origin: Vector, offset: Vector) -> Self {
        Self { origin, offset }
    }

    pub fn new_from(size_pixels: Vector2<u32>) -> Self {
        Self::new(Vector::zero(), Vector::new(size_pixels.x as f32, size_pixels.y as f32))
    }

    pub fn shift_shape(&self, shape: &Shape) -> Shape {
        shape.scale_2d(&self.offset).translate(&self.origin)
    }

    pub fn translate(&self, v: &Vector) -> Vector {
        self.origin + self.offset.scale2d(v)
    }

    pub fn reverse_translate_to_unit(&self, v: &Vector) -> Vector {
        let v1 = *v - self.origin;
        let x = v1.x / self.offset.x;
        let y = v1.y / self.offset.y;
        Vector::new(x, y)
    }

    pub fn transform_by_frame(&self, f: &TransFrame) -> Frame {
        let new_origin = self.translate(&f.origin);
        Frame::new(
            new_origin,
            self.translate(&f.corner) - new_origin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rectangle_at_origin() -> Frame {
        let origin = Vector::new(0.0, 0.0);
        let offset = Vector::new(12.0, 14.0);
        Frame::new(origin, offset)
    }

    fn rectangle() -> Frame {
        let origin = Vector::new(10.0, 20.0);
        let offset = Vector::new(12.0, 14.0);
        Frame::new(origin, offset)
    }

    fn very_close(given: &Vector, expected: &Vector) {
        assert!(given.is_very_close(expected));
    }

    #[test]
    fn translate() {
        let frame = rectangle_at_origin();
        assert_eq!(frame.translate(&Vector::new(0.0, 0.0)), Vector::new(0.0, 0.0));
        assert_eq!(frame.translate(&Vector::new(1.0, 1.0)), Vector::new(12.0, 14.0));
        assert_eq!(frame.translate(&Vector::new(0.5, 0.5)), Vector::new(6.0, 7.0));
        assert_eq!(frame.translate(&Vector::new(0.5, 0.1)), Vector::new(6.0, 1.4));

        let frame = rectangle();
        assert_eq!(frame.translate(&Vector::new(0.0, 0.0)), Vector::new(10.0, 20.0));
        assert_eq!(frame.translate(&Vector::new(1.0, 1.0)), Vector::new(22.0, 34.0));
        assert_eq!(frame.translate(&Vector::new(0.5, 0.5)), Vector::new(16.0, 27.0));
        assert_eq!(frame.translate(&Vector::new(0.5, 0.1)), Vector::new(16.0, 21.4));
    }

    #[test]
    fn reverse_translation() {
        let frame = rectangle_at_origin();
        assert_eq!(frame.reverse_translate_to_unit(&Vector::new(0.0, 0.0)), Vector::new(0.0, 0.0));
        assert_eq!(frame.reverse_translate_to_unit(&Vector::new(12.0, 14.0)), Vector::new(1.0, 1.0));
        assert_eq!(frame.reverse_translate_to_unit(&Vector::new(6.0, 7.0)), Vector::new(0.5, 0.5));
        assert_eq!(frame.reverse_translate_to_unit(&Vector::new(1.2, 1.4)), Vector::new(0.1, 0.1));

        let frame = rectangle();
        assert_eq!(frame.reverse_translate_to_unit(&Vector::new(10.0, 20.0)), Vector::new(0.0, 0.0));
        assert_eq!(frame.reverse_translate_to_unit(&Vector::new(22.0, 34.0)), Vector::new(1.0, 1.0));
        assert_eq!(frame.reverse_translate_to_unit(&Vector::new(16.0, 27.0)), Vector::new(0.5, 0.5));
        very_close(&frame.reverse_translate_to_unit(&Vector::new(11.2, 21.4)),
                   &Vector::new(0.1, 0.1));
    }

    #[test]
    fn transform_by_frame() {
        let frame = rectangle_at_origin();
        assert_eq!(frame.transform_by_frame(&TransFrame::unit()), frame);
        assert_eq!(frame.transform_by_frame(&TransFrame::left_half()),
                   Frame::new(Vector::zero(), Vector::new(6.0, 14.0)));
        assert_eq!(frame.transform_by_frame(&TransFrame::right_half()),
                   Frame::new(Vector::new(6.0, 0.0), Vector::new(6.0, 14.0)));
        assert_eq!(frame.transform_by_frame(&TransFrame::top_half()),
                   Frame::new(Vector::new(0.0, 0.0), Vector::new(12.0, 7.0)));
        assert_eq!(frame.transform_by_frame(&TransFrame::bottom_half()),
                   Frame::new(Vector::new(0.0, 7.0), Vector::new(12.0, 7.0)));
        assert_eq!(frame.transform_by_frame(&TransFrame::reflect_x()),
                   Frame::new(Vector::new(12.0, 0.0), Vector::new(0.0, 14.0)));
        assert_eq!(frame.transform_by_frame(&TransFrame::reflect_y()),
                   Frame::new(Vector::new(0.0, 14.0), Vector::new(12.0, -14.0)));
    }
}