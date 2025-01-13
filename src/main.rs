use crate::shape_shifter::ShapeShifter;

// use picture_language::picture;
pub mod painter;
pub mod transformed_painter;
pub mod composite_painter;
pub mod colour_selector_painter;
pub mod thickness_selector_painter;
pub mod frame;
pub mod trans_frame;
pub mod carpet;

pub mod picture;
pub mod segment;
pub mod shape;
pub mod vector;
pub mod shape_shifter;
mod window_handler;

fn main() {
    let mut shape_shifter = ShapeShifter::new();
    window_handler::run_picture_window(picture::make_picture(&mut shape_shifter), shape_shifter);
}

#[cfg(test)]
mod tests {
    // So we can run all the tests at once
    #[test]
    fn t() {
        assert_eq!(true, true);
    }
}