use crate::shape::Shape;
use crate::trans_frame::TransFrame;
use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct Frame {
    origin: Vector,
    edge1: Vector,
    edge2: Vector,
}

impl Frame {
    pub fn new(origin: Vector, edge1: Vector, edge2: Vector) -> Self {
        // assert!(edge)
        Self { origin, edge1, edge2 }
    }

    pub fn shift_shape(&self, shape: &Shape) -> Shape {
        shape.scale_2d(&self.edge1, &self.edge2).translate(&self.origin)
    }

    pub fn translate(&self, v: &Vector) -> Vector {
        self.origin + self.edge1.scale(v.x) + self.edge2.scale(v.y)
    }

    pub fn transform_by_frame(&self, f: &TransFrame) -> Frame {
        let new_origin = self.translate(&f.origin);
        Frame::new(
            new_origin,
            self.translate(&f.corner1) - new_origin,
            self.translate(&f.corner2) - new_origin)
    }

    pub fn to_unit(&self, point: &Vector) -> Vector {
        let x = self.distance(self.edge1, point);
        let y = self.distance(self.edge2, point);
        self.reverse_translate(&Vector::new(x, y))
    }

    pub fn reverse_translate(&self, v: &Vector) -> Vector {
        let x1 = self.edge1.x;
        let y1 = self.edge1.y;
        let x2 = self.edge2.x;
        let y2 = self.edge2.y;
        println!("({},{})  ({x1},{y1})  ({x2},{y2})", v.x, v.y);
        let y = (y2 * v.x - x1 * v.y) / (x2 * y2 - x1 * y1);
        let x = (v.x - x2 * v.y) / x1;
        Vector::new(x, y)
    }

    // pub fn reverse_translate(&self, v: &Vector) -> Vector {
    //     let sum = self.edge1 + self.edge2;
    //     Vector::new(v.x / sum.x, v.y / sum.y)
    // }
    //
    pub fn distance(&self, edge: Vector, point: &Vector) -> f32 {
        let x0 = point.x;
        let y0 = point.y;
        let x1 = self.origin.x;
        let x2 = self.origin.x + edge.x;
        let y1 = self.origin.y;
        let y2 = y1 + edge.y;
        let numerator = ((x2 - x1) * (y1 - y0) - (x1 - x0) * (y2 - y1)).abs();
        let denominator = ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt();
        numerator / denominator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rectangle() -> Frame {
        let origin = Vector::new(0.0, 0.0);
        let edge1 = Vector::new(12.0, 0.0);
        let edge2 = Vector::new(0.0, 14.0);
        Frame::new(origin, edge1, edge2)
    }

    fn parallelogram() -> Frame {
        let origin = Vector::new(0.0, 0.0);
        let edge1 = Vector::new(12.0, 2.0);
        let edge2 = Vector::new(3.0, 14.0);
        Frame::new(origin, edge1, edge2)
    }

    #[test]
    fn reverse_translation_rectangle() {
        let frame = rectangle();
        assert_eq!(frame.reverse_translate(&Vector::new(0.0, 0.0)), Vector::new(0.0, 0.0));
        assert_eq!(frame.reverse_translate(&Vector::new(12.0, 14.0)), Vector::new(1.0, 1.0));
        assert_eq!(frame.reverse_translate(&Vector::new(6.0, 7.0)), Vector::new(0.5, 0.5));

        assert_eq!(frame.reverse_translate(&Vector::new(1.2, 1.4)), Vector::new(0.1, 0.1));
        assert_eq!(frame.translate(&Vector::new(0.1, 0.1)), Vector::new(1.2, 1.4));
    }

    #[test]
    fn reverse_translation_parallelogram() {
        let frame = parallelogram();
        assert_eq!(frame.reverse_translate(&Vector::new(0.0, 0.0)), Vector::new(0.0, 0.0));

        let p0 = Vector::new(1.0, 1.0);
        let p0_transformed = Vector::new(15.0, 16.0);
        assert_eq!(frame.translate(&p0), p0_transformed);
        assert_eq!(frame.reverse_translate(&p0_transformed), p0);

        let p1 = Vector::new(0.5, 0.5);
        let p1_transformed = Vector::new(7.5, 8.0);
        assert_eq!(frame.translate(&p1), p1_transformed);
        assert_eq!(frame.reverse_translate(&p1_transformed), p1);

        let p2 = Vector::new(0.5, 0.8);
        let p2_transformed = Vector::new(8.4, 12.2);
        assert_eq!(frame.translate(&p2), p2_transformed);
        assert_eq!(frame.reverse_translate(&p2_transformed), p2);

        // let p1 = Vector::new(7.5, 8.0);
        // let p1_result = Vector::new(0.5, 0.5);
        // assert_eq!(frame.reverse_translate(&p1), p1_result);
        // assert_eq!(frame.translate(&p1_result), p1);
        //
        // assert_eq!(frame.translate(&Vector::new(0.1, 0.2)), Vector::new(1.8000001, 3.0));
        // assert_eq!(frame.reverse_translate(&Vector::new(1.8000001, 3.0)), Vector::new(0.1, 0.2));


        // let p2 = Vector::new(5.5, 3.5);
        // let p2_result = Vector::new(0.36666667, 0.21875);
        // assert_eq!(frame.reverse_translate(&p2), p2_result);
        // assert_eq!(frame.translate(&p2_result), p2);
    }

    #[test]
    fn translate_rectangle() {
        let frame = rectangle();
        assert_eq!(frame.translate(&Vector::new(0.0, 0.0)), Vector::new(0.0, 0.0));
        assert_eq!(frame.translate(&Vector::new(1.0, 1.0)), Vector::new(12.0, 14.0));
        assert_eq!(frame.translate(&Vector::new(0.5, 0.5)), Vector::new(6.0, 7.0));
        assert_eq!(frame.translate(&Vector::new(0.5, 0.1)), Vector::new(6.0, 1.4));
    }

    #[test]
    fn translate_parallelogram() {
        let origin = Vector::new(0.0, 0.0);
        let edge1 = Vector::new(12.0, 2.0);
        let edge2 = Vector::new(3.0, 14.0);
        let frame = Frame::new(origin, edge1, edge2);
        assert_eq!(frame.translate(&Vector::new(0.0, 0.0)), Vector::new(0.0, 0.0));
        assert_eq!(frame.translate(&Vector::new(1.0, 1.0)), Vector::new(15.0, 16.0));
        assert_eq!(frame.translate(&Vector::new(0.5, 0.5)), Vector::new(7.5, 8.0));
        assert_eq!(frame.translate(&Vector::new(0.5, 0.1)), Vector::new(6.3, 2.4));
    }

    #[test]
    fn distance() {
        let origin = Vector::new(0.0, 0.0);
        let edge1 = Vector::new(12.0, 0.0);
        let edge2 = Vector::new(0.0, 14.0);
        let frame = Frame::new(origin, edge1, edge2);
        let point = Vector::new(1.0, 0.0);
        assert_eq!(frame.distance(frame.edge1, &point), 0.0);
        assert_eq!(frame.distance(frame.edge2, &point), 1.0);
    }

    #[test]
    fn to_unit_with_square() {
        let frame = Frame::new(
            Vector::new(0.0, 0.0),
            Vector::new(12.0, 0.0),
            Vector::new(0.0, 12.0));
        assert_eq!(frame.to_unit(&Vector::new(0.0, 0.0)), Vector::new(0.0, 0.0));
        assert_eq!(frame.to_unit(&Vector::new(12.0, 12.0)), Vector::new(1.0, 1.0));
        assert_eq!(frame.to_unit(&Vector::new(12.0 / 2.0, 12.0 / 2.0)), Vector::new(0.5, 0.5));
        assert_eq!(frame.to_unit(&Vector::new(12.0 / 10.0, 12.0 / 10.0)), Vector::new(0.1, 0.1));
    }

    #[test]
    fn to_unit_with_rectangle() {
        let frame = Frame::new(
            Vector::new(0.0, 0.0),
            Vector::new(12.0, 0.0),
            Vector::new(0.0, 16.0));
        assert_eq!(frame.to_unit(&Vector::new(0.0, 0.0)), Vector::new(0.0, 0.0));
        assert_eq!(frame.to_unit(&Vector::new(12.0, 16.0)), Vector::new(1.0, 1.0));
        assert_eq!(frame.to_unit(&Vector::new(12.0 / 2.0, 16.0 / 2.0)), Vector::new(0.5, 0.5));
        assert_eq!(frame.to_unit(&Vector::new(12.0 / 10.0, 16.0 / 10.0)), Vector::new(0.1, 0.1));
    }
}