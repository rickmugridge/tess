use crate::vector::Vector;
use speedy2d::color::Color;
use crate::segment::{Segment};

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Line(Segment, f32, Color),
    PolyLine(Vec<Segment>, f32, Color),
    Circle(Vector, f32, Color), // radius
}

impl Shape {
    pub fn new_line(segment: Segment, thickness: f32, colour: Color) -> Shape {
        Shape::Line(segment, thickness, colour)
    }

    pub fn new_poly_line(s: Vec<Segment>, thickness: f32, colour: Color) -> Shape {
        Shape::PolyLine(s, thickness, colour)
    }

    pub fn new_circle(centre: &Vector, radius: f32, colour: Color) -> Shape {
        Shape::Circle(*centre, radius, colour)
    }

    pub fn scale_2d(&self, offset: &Vector) -> Shape {
        match self {
            Shape::Line(ref segment, thickness, colour) =>
                Shape::new_line(segment.scale2d(offset), *thickness, *colour),
            Shape::PolyLine(segments, thickness, colour) => {
                let segments = segments.iter()
                    .map(|s| s.scale2d(offset)).collect();
                Shape::new_poly_line(segments, *thickness, *colour)
            }
            Shape::Circle(centre, radius, colour) =>
                Shape::new_circle(&centre, *radius, *colour), // todo scale radius
        }
    }

    pub fn translate(&self, v: &Vector) -> Shape {
        match self {
            Shape::Line(ref segment, thickness, colour) =>
                Shape::new_line(segment.translate(v), *thickness, *colour),
            Shape::PolyLine(segments, thickness, colour) => {
                let segments = segments.iter()
                    .map(|s| s.translate(v)).collect();
                Shape::new_poly_line(segments, *thickness, *colour)
            }
            Shape::Circle(centre, radius, colour) =>
                Shape::new_circle(&centre.translate(v), *radius, *colour),
        }
    }
}