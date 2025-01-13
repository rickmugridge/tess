use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use speedy2d::dimen::Vec2;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)?;
        Ok(())
    }
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }

    pub fn new_from_pair(pair: (f32, f32)) -> Self {
        Vector::new(pair.0, pair.1)
    }

    pub fn zero() -> Self {
        Vector { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Self {
        Vector { x: 1.0, y: 1.0 }
    }

    pub fn scale(&self, s: f32) -> Self {
        Vector { x: self.x * s, y: self.y * s }
    }

    pub fn divided_by(&self, s: f32) -> Self {
        let x = if self.x == 0.0 { 0.0 } else { s / self.x };
        let y = if self.y == 0.0 { 0.0 } else { s / self.y };
        Vector { x, y }
    }

    pub fn scale2d(&self, offset: &Vector) -> Vector {
        Vector::new(self.x * offset.x, self.y * offset.y)
    }

    pub fn translate(&self, v: &Vector) -> Vector {
        Vector::new(self.x + v.x, self.y + v.y)
    }

    pub fn translate_x(&self, x: f32) -> Vector {
        Vector::new(self.x + x, self.y)
    }

    pub fn translate_y(&self, y: f32) -> Vector {
        Vector::new(self.x, self.y + y)
    }

    pub fn is_close(&self, other: &Vector) -> bool {
        (self.x - other.x).abs() <= 2.0 || (self.x - other.x).abs() <= 2.0 // todo reconsider this value
    }

    pub fn is_very_close(&self, other: &Vector) -> bool {
        (self.x - other.x) < 0.00001 && (self.y - other.y) < 0.00001 // todo reconsider this value
    }

    pub fn inside_unit(&self) -> bool {
        self.x >= 0.0 && self.x <= 1.0 && self.y >= 0.0 && self.y <= 1.0
    }

    pub fn to_vec2(&self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
}

impl From<Vector> for Vec2 {
    fn from(value: Vector) -> Self {
        Vec2::new(value.x, value.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(Vector::new(1.0, 2.0) + Vector::new(3.0, 4.0),
                   Vector::new(4.0, 6.0));
    }

    #[test]
    fn subtract() {
        assert_eq!(Vector::new(1.0, 2.0) - Vector::new(3.0, 4.0),
                   Vector::new(-2.0, -2.0));
    }

    #[test]
    fn scale() {
        assert_eq!(Vector::new(1.0, 2.0).scale(10.0),
                   Vector::new(10.0, 20.0));
    }

    #[test]
    fn scale2d_with_zero() {
        assert_eq!(Vector::zero().scale2d(&Vector::new(100.0, 200.0)),
                   Vector::zero());
    }

    #[test]
    fn scale2d_with_one() {
        assert_eq!(Vector::one().scale2d(&Vector::new(100.0, 200.0)),
                   Vector::new(100.0, 200.0));
    }

    #[test]
    fn scale2d_with_half() {
        let point = Vector::new(0.5, 0.5);
        let offset = Vector::new(100.0, 200.0);
        assert_eq!(point.scale2d(&offset),
                   Vector::new(50.0, 100.0));
    }

    #[test]
    fn is_close() {
        let point = Vector::new(20.0, 30.0);
        assert_eq!(point.is_close(&point), true);
        assert_eq!(point.is_close(&(point + Vector::new(1.0, 1.0))), true);
        assert_eq!(point.is_close(&(point + Vector::new(5.0, 0.0))), false);
    }
}