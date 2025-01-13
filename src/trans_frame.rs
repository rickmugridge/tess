use crate::vector::Vector;

#[derive(Debug, PartialEq, Clone)]
pub struct TransFrame {
    pub origin: Vector,
    pub corner: Vector,
    offset: Vector,
}

impl TransFrame {
    pub fn new(origin: Vector, corner: Vector) -> Self {
        Self { origin, corner, offset: Vector::new(corner.x - origin.x, corner.y - origin.y) }
    }

    pub fn reverse_translate(&self, v: &Vector) -> Vector {
        let v1 = *v - self.origin;
        // println!("TransFrame@15 origin:{} offset:{} v:{} v1:{}", self.origin, self.offset, v, v1);
        let x = if self.offset.x > 0.0 { v1.x / self.offset.x } else { (1.0 - v.x) / self.offset.x.abs() };
        let y = if self.offset.y > 0.0 { v1.y / self.offset.y } else { (1.0 - v.y) / self.offset.y.abs() };
        // println!("TransFrame@22 offset:{} v:{} x:{} y:{}", self.offset, v, x, y);
        Vector::new(x, y)
    }

    pub fn unit() -> Self {
        Self::new(
            Vector::zero(),
            Vector::new(1.0, 1.0))
    }

    pub fn left_right_halves() -> (Self, Self) {
        (Self::left_half(), Self::right_half())
    }

    pub fn top_bottom_halves() -> (Self, Self) {
        (Self::top_half(), Self::bottom_half())
    }

    pub fn left_right(left_fraction: f32) -> (Self, Self) {
        let left = Self::new(
            Vector::zero(),
            Vector::new(left_fraction, 1.0));
        let right = Self::new(
            Vector::new(left_fraction, 0.0),
            Vector::new(1.0, 1.0));
        (left, right)
    }

    pub fn left_half() -> Self {
        Self::new(
            Vector::zero(),
            Vector::new(0.5, 1.0))
    }

    pub fn right_half() -> Self {
        Self::new(
            Vector::new(0.5, 0.0),
            Vector::new(1.0, 1.0))
    }

    pub fn top_half() -> Self {
        Self::new(
            Vector::zero(),
            Vector::new(1.0, 0.5))
    }

    pub fn bottom_half() -> Self {
        Self::new(
            Vector::new(0.0, 0.5),
            Vector::new(1.0, 1.0))
    }

    pub fn reflect_x() -> Self {
        Self::new(
            Vector::new(1.0, 0.0),
            Vector::new(0.0, 1.0))
    }

    pub fn reflect_y() -> Self {
        Self::new(
            Vector::new(0.0, 1.0),
            Vector::new(1.0, 0.0))
    }

    pub fn tessellate_x(count: usize) -> Vec<TransFrame> {
        let delta_x = 1.0 / count as f32;
        let result: Vec<TransFrame> = (0..count).map(|i| {
            let offset = delta_x * i as f32;
            Self::new(
                Vector::new(offset, 0.0),
                Vector::new(delta_x * (i + 1) as f32, 1.0))
        }).collect();
        result
    }

    pub fn tessellate_y(count: usize) -> Vec<TransFrame> {
        let delta_y = 1.0 / count as f32;
        let result: Vec<TransFrame> = (0..count).map(|i| {
            let offset = delta_y * i as f32;
            Self::new(
                Vector::new(0.0, offset),
                Vector::new(1.0, delta_y * (i + 1) as f32))
        }).collect();
        result
    }

    /*    pub fn rotate_90() -> Self {
            Self::new(
                Vector::new(1.0, 0.0),
                Vector::new(0.0, 1.0))
           }

          pub fn rotate_180() -> Self {
                Self::new(
                    Vector::new(1.0, 1.0),
                    Vector::new(0.0, 1.0),
                    Vector::new(1.0, 0.0))
            }

            pub fn rotate_270() -> Self {
                Self::new(
                    Vector::new(0.0, 1.0),
                    Vector::zero(),
                    Vector::new(1.0, 1.0))
            }
        */
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v25() -> Vector { Vector::new(0.25, 0.25) }

    fn v75() -> Vector { Vector::new(0.75, 0.75) }

    #[test]
    fn reverse_translate_unit() {
        let trans = TransFrame::unit();
        assert_eq!(trans.reverse_translate(&Vector::zero()), Vector::zero());
        assert_eq!(trans.reverse_translate(&v25()), v25());
        assert_eq!(trans.reverse_translate(&v75()), Vector::new(0.75, 0.75));
        assert_eq!(trans.reverse_translate(&Vector::one()), Vector::one());
    }

    #[test]
    fn reverse_translate_left() {
        let trans = TransFrame::left_half();
        assert_eq!(trans.reverse_translate(&Vector::zero()), Vector::zero());
        assert_eq!(trans.reverse_translate(&v25()), Vector::new(0.5, 0.25));
        assert_eq!(trans.reverse_translate(&v75()), Vector::new(1.5, 0.75));
        assert_eq!(trans.reverse_translate(&Vector::one()), Vector::new(2.0, 1.0));
    }

    #[test]
    fn reverse_translate_right() {
        let trans = TransFrame::right_half();
        assert_eq!(trans.reverse_translate(&Vector::zero()), Vector::new(-1.0, 0.0));
        assert_eq!(trans.reverse_translate(&v25()), Vector::new(-0.5, 0.25));
        assert_eq!(trans.reverse_translate(&v75()), Vector::new(0.5, 0.75));
        assert_eq!(trans.reverse_translate(&Vector::one()), Vector::one());
    }

    #[test]
    fn reverse_translate_top() {
        let trans = TransFrame::top_half();
        assert_eq!(trans.reverse_translate(&Vector::zero()), Vector::zero());
        assert_eq!(trans.reverse_translate(&v25()), Vector::new(0.25, 0.5));
        assert_eq!(trans.reverse_translate(&v75()), Vector::new(0.75, 1.5));
        assert_eq!(trans.reverse_translate(&Vector::one()), Vector::new(1.0, 2.0));
    }

    #[test]
    fn reverse_translate_bottom() {
        let trans = TransFrame::bottom_half();
        assert_eq!(trans.reverse_translate(&Vector::zero()), Vector::new(0.0, -1.0));
        assert_eq!(trans.reverse_translate(&v25()), Vector::new(0.25, -0.5));
        assert_eq!(trans.reverse_translate(&v75()), Vector::new(0.75, 0.5));
        assert_eq!(trans.reverse_translate(&Vector::one()), Vector::one());
    }

    #[test]
    fn reverse_translate_reflect_x() {
        let trans = TransFrame::reflect_x();
        assert_eq!(trans.reverse_translate(&Vector::zero()), Vector::new(1.0, 0.0));
        assert_eq!(trans.reverse_translate(&v25()), Vector::new(0.75, 0.25));
        assert_eq!(trans.reverse_translate(&v75()), Vector::new(0.25, 0.75));
        assert_eq!(trans.reverse_translate(&Vector::one()), Vector::new(0.0, 1.0));
    }

    #[test]
    fn reverse_translate_reflect_y() {
        let trans = TransFrame::reflect_y();
        assert_eq!(trans.reverse_translate(&Vector::zero()), Vector::new(0.0, 1.0));
        assert_eq!(trans.reverse_translate(&v25()), Vector::new(0.25, 0.75));
        assert_eq!(trans.reverse_translate(&v75()), Vector::new(0.75, 0.25));
        assert_eq!(trans.reverse_translate(&Vector::one()), Vector::new(1.0, 0.0));
    }

    #[test]
    fn reverse_translate_tessellate_x_1() {
        let trans = TransFrame::tessellate_x(1);
        assert_eq!(trans[0].reverse_translate(&Vector::zero()), Vector::new(0.0, 0.0));
        assert_eq!(trans[0].reverse_translate(&v25()), Vector::new(0.25, 0.25));
        assert_eq!(trans[0].reverse_translate(&v75()), Vector::new(0.75, 0.75));
        assert_eq!(trans[0].reverse_translate(&Vector::one()), Vector::new(1.0, 1.0));
    }

    #[test]
    fn reverse_translate_tessellate_x_2() {
        let trans = TransFrame::tessellate_x(2);
        assert_eq!(trans[0].reverse_translate(&Vector::zero()), Vector::new(0.0, 0.0));
        assert_eq!(trans[0].reverse_translate(&v25()), Vector::new(0.5, 0.25));
        assert_eq!(trans[0].reverse_translate(&v75()), Vector::new(1.5, 0.75));
        assert_eq!(trans[0].reverse_translate(&Vector::one()), Vector::new(2.0, 1.0));

        assert_eq!(trans[1].reverse_translate(&Vector::zero()), Vector::new(-1.0, 0.0));
        assert_eq!(trans[1].reverse_translate(&v25()), Vector::new(-0.5, 0.25));
        assert_eq!(trans[1].reverse_translate(&v75()), Vector::new(0.5, 0.75));
        assert_eq!(trans[1].reverse_translate(&Vector::one()), Vector::new(1.0, 1.0));
    }
}