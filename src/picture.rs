use std::rc::Rc;
use speedy2d::color::Color;
use crate::carpet::{carpet, CarpetLeftToRight, CarpetTopToBottom};
use crate::colour_selector_painter::ColourSelectorPainter;
use crate::composite_painter::CompositePainter;
use crate::segment::Segment;
use crate::vector::Vector;
use crate::painter::{Painter, UnitPainter};
use crate::shape::Shape;
use crate::shape_shifter::ShapeShifter;
use crate::thickness_selector_painter::ThicknessSelectorPainter;
use crate::trans_frame::TransFrame;
use crate::transformed_painter::TransformedPainter;

pub fn make_picture(shape_shifter: &mut ShapeShifter) -> Rc<Box<dyn Painter>> {
    let shapes = vec![
        Shape::new_poly_line(partial_outer_bounds(), 1.0, Color::LIGHT_GRAY),
        // Shape::new_poly_line(diamond(), 1.0, Color::RED),
        Shape::new_poly_line(half_cross(), 1.0, Color::LIGHT_GRAY),
    ];
    let top_painter1 = UnitPainter::new(shape_shifter.add(shapes.clone()));
    let top_painter2 = UnitPainter::new(shape_shifter.add(shapes.clone()));

    let centre_unit_painter = UnitPainter::new(shape_shifter.add(shapes.clone()));
    let centre_unit_painter = CompositePainter::double_reflect(centre_unit_painter);
    let centre_painter = CompositePainter::tessellate_x_y(centre_unit_painter, 3, 3);

    let left_painter1 = UnitPainter::new(shape_shifter.add(shapes.clone()));
    let left_painter2 = UnitPainter::new(shape_shifter.add(shapes.clone()));


    let carpet = carpet(
        vec![(left_painter1, 8, 0.05), (left_painter2, 8, 0.1)],
        centre_painter,
        vec![(top_painter1, 8, 0.08), (top_painter2, 8, 0.1)],
    );

    let (left, right) = TransFrame::left_right(0.1);
    let picture = CompositePainter::new(vec![
        TransformedPainter::new(make_panels(shape_shifter), left),
        TransformedPainter::new(carpet, right),
    ]);
    picture
}

fn make_panels(shape_shifter: &mut ShapeShifter) -> Rc<Box<dyn Painter>> {
    CompositePainter::new(vec![
        TransformedPainter::new(make_colour_panel(shape_shifter),
                                TransFrame::top_half()),
        TransformedPainter::new(make_thickness_panel(shape_shifter),
                                TransFrame::bottom_half()),
    ])
}

fn make_colour_panel(shape_shifter: &mut ShapeShifter) -> Rc<Box<dyn Painter>> {
    let painter = ColourSelectorPainter::make_panel(vec![
        thick_outline(Color::BLUE, shape_shifter),
        thick_outline(Color::RED, shape_shifter),
        thick_outline(Color::GREEN, shape_shifter),
        thick_outline(Color::GRAY, shape_shifter),
        thick_outline(Color::BLACK, shape_shifter),
        thick_outline(Color::CYAN, shape_shifter),
        thick_outline(Color::MAGENTA, shape_shifter),
    ]);
    TransformedPainter::bordered(
        painter, Vector::new(0.2, 0.1), Vector::new(0.8, 0.9))
}

fn make_thickness_panel(shape_shifter: &mut ShapeShifter) -> Rc<Box<dyn Painter>> {
    let painter = ThicknessSelectorPainter::make_panel(vec![
        thick_cross(1.0, shape_shifter),
        thick_cross(2.0, shape_shifter),
        thick_cross(3.0, shape_shifter),
        thick_cross(4.0, shape_shifter),
        thick_cross(5.0, shape_shifter),
        thick_cross(6.0, shape_shifter),
        thick_cross(10.0, shape_shifter),
    ]);
    TransformedPainter::bordered(
        painter, Vector::new(0.2, 0.1), Vector::new(0.8, 0.9))
}

fn thick_outline(colour: Color, shape_shifter: &mut ShapeShifter) -> (usize, Color) {
    let shape_index = shape_shifter.add(
        vec![Shape::new_poly_line(outer_bounds(), 10.0, colour)]);
    (shape_index, colour)
}

fn thick_cross(thickness: f32, shape_shifter: &mut ShapeShifter) -> (usize, f32) {
    let shape_index = shape_shifter.add(
        vec![Shape::new_poly_line(cross(), thickness, Color::BLACK)]);
    (shape_index, thickness)
}

pub fn make_picture22(shape_shifter: &mut ShapeShifter) -> Rc<Box<dyn Painter>> {
    let shapes = make_shape();
    let top_painter = UnitPainter::new(shape_shifter.add(shapes.clone()));
    let top_painter = CompositePainter::tessellate_x(top_painter, 4);

    let centre_unit_painter = UnitPainter::new(shape_shifter.add(shapes.clone()));
    let centre_unit_painter = CompositePainter::double_reflect(centre_unit_painter);

    let centre_painter = CompositePainter::tessellate_x_y(centre_unit_painter, 3, 3);
    let bottom_painter = TransformedPainter::new(top_painter.clone(),
                                                 TransFrame::reflect_y());
    let down_composite = CarpetTopToBottom::make(0.2)
        .compose(top_painter, centre_painter, bottom_painter);

    let left_painter = UnitPainter::new(shape_shifter.add(shapes.clone()));
    let left_painter = CompositePainter::tessellate_y(left_painter, 4);
    let right_painter = TransformedPainter::new(left_painter.clone(),
                                                TransFrame::reflect_x());
    let across_composite = CarpetLeftToRight::make(0.2)
        .compose(left_painter.clone(), down_composite, right_painter);


    let picture = across_composite;
    picture
}

pub fn make_picture_reflect(shape_shifter: &mut ShapeShifter) -> Rc<Box<dyn Painter>> {
    let shapes = make_shape();
    let index = shape_shifter.add(shapes);
    let painter1 = UnitPainter::new(index);
    let painter2 = CompositePainter::tessellate_x(painter1, 4);
    let painter3 = TransformedPainter::new(painter2.clone(),
                                           TransFrame::reflect_y());
    let picture = CompositePainter::above(painter2, painter3);
    picture
}

pub fn make_picture_unit(shape_shifter: &mut ShapeShifter) -> Rc<Box<dyn Painter>> {
    let shapes = make_shape();
    let index = shape_shifter.add(shapes);
    UnitPainter::new(index)
}

fn make_shape() -> Vec<Shape> {
    vec![
        Shape::new_poly_line(partial_outer_bounds(), 1.0, Color::LIGHT_GRAY),
        // Shape::new_poly_line(diamond(), 1.0, Color::RED),
        Shape::new_poly_line(half_cross(), 1.0, Color::LIGHT_GRAY),
    ]
}

fn partial_outer_bounds() -> Vec<Segment> {
    let top_right = Vector::new(1.0, 0.0);
    let _bottom_left = Vector::new(0.0, 1.0);
    Segment::open_path(&vec![Vector::zero(), top_right, Vector::one(), Vector::zero()])
    // Segment::open_path(&vec![Vector::zero(), top_right, Vector::one(), bottom_left, Vector::zero()])
}

fn outer_bounds() -> Vec<Segment> {
    let top_right = Vector::new(1.0, 0.0);
    let bottom_left = Vector::new(0.0, 1.0);
    Segment::open_path(&vec![Vector::zero(), top_right, Vector::one(), bottom_left, Vector::zero()])
}

fn cross() -> Vec<Segment> {
    let top_right = Vector::new(1.0, 0.0);
    let bottom_left = Vector::new(0.0, 1.0);
    vec![
        Segment::new(top_right, bottom_left),
        Segment::new(Vector::zero(), Vector::new(1.0, 1.0)),
    ]
}
/*fn diamond() -> Vec<Segment> {
    Segment::open_path(&vec![
        Vector::new(0.0, 0.5),
        Vector::new(0.5, 0.0),
        Vector::new(1.0, 0.5),
        Vector::new(0.5, 1.0),
        Vector::new(0.0, 0.5),
    ])
}
*/

fn half_cross() -> Vec<Segment> {
    vec![
        Segment::new(Vector::zero(), Vector::new(1.0, 1.0)),
    ]
}
