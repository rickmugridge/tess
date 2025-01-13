use std::rc::Rc;
use speedy2d::color::Color;
use speedy2d::dimen::{Vec2, UVec2, IVec2};
use speedy2d::window::{WindowHandler, WindowHelper, MouseButton, WindowStartupInfo, WindowSize, WindowPosition, WindowCreationOptions};
use speedy2d::{Graphics2D, Window};
use crate::frame::Frame;
use crate::painter::{Painter, PainterResponse};
use crate::segment::Segment;
use crate::shape::Shape;
use crate::shape_shifter::{Rendering, ShapeShifter};
use crate::vector::Vector;

pub fn run_picture_window(painter: Rc<Box<dyn Painter>>, shape_shifter: ShapeShifter) {
    let window_size = UVec2::new(1000, 1000);
    let options = WindowCreationOptions::new_windowed(
        WindowSize::PhysicalPixels(window_size),
        Some(WindowPosition::PrimaryMonitorPixelsFromTopLeft(IVec2::new(20, 30))));
    let window = Window::new_with_options("Tess", options).unwrap();
    let frame = Frame::new_from(window_size);
    window.run_loop(PictureWindowHandler::new(painter, frame, shape_shifter))
}

pub struct PictureWindowHandler {
    painter: Rc<Box<dyn Painter>>,
    frame: Frame,
    shape_shifter: ShapeShifter,
    mouse_position: Option<Vector>,
    dragging: bool,
    scale_factor: f32,
    pen_thickness: f32,
    pen_colour: Color,
    logging_count_left: usize,
}

impl PictureWindowHandler {
    fn new(painter: Rc<Box<dyn Painter>>, frame: Frame, shape_shifter: ShapeShifter) -> Self {
        PictureWindowHandler {
            painter,
            frame,
            shape_shifter,
            mouse_position: None,
            dragging: false,
            scale_factor: 1.0,
            pen_thickness: 2.0,
            pen_colour: Color::BLUE,
            logging_count_left: 10000,
        }
    }
}

impl<'a> WindowHandler for PictureWindowHandler {
    fn on_start(&mut self, _helper: &mut WindowHelper, info: WindowStartupInfo) {
        println!("on_start {:?}", info);
    }

    fn on_resize(&mut self, helper: &mut WindowHelper, size_pixels: UVec2) {
        // println!("on_resize {:?}", size_pixels);
        self.frame = Frame::new_from(size_pixels);
        self.scale_factor = 1.0;
        helper.request_redraw();
    }

    fn on_mouse_grab_status_changed(&mut self, _helper: &mut WindowHelper<>, mouse_grabbed: bool) {
        println!("on_mouse_grab_status_changed {}", mouse_grabbed);
    }

    fn on_fullscreen_status_changed(&mut self,
                                    _helper: &mut WindowHelper<>,
                                    fullscreen: bool) {
        println!("on_fullscreen_status_changed {}", fullscreen);
    }

    fn on_scale_factor_changed(&mut self, helper: &mut WindowHelper<>, scale_factor: f64) {
        println!("on_scale_factor_changed {}", scale_factor);
        self.scale_factor = scale_factor as f32;
        helper.request_redraw();
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::WHITE);
        let mut renderings: Vec<Rendering> = vec![];
        self.painter.shapes_to_draw(self.frame.clone(), &mut renderings);
        let mut shapes_to_render: Vec<Shape> = vec![];
        self.shape_shifter.render(renderings, &mut shapes_to_render);
        for shape in &shapes_to_render {
            match shape {
                Shape::Line(ref segment, thickness, colour) =>
                    {
                        // println!("window_handler@61 render line segment {:?}", segment);
                        graphics.draw_line(segment.start.to_vec2(), segment.end.to_vec2(),
                                           *thickness, *colour);
                    }
                Shape::PolyLine(segments, thickness, colour) => {
                    segments.iter().for_each(|s|
                        graphics.draw_line(s.start.to_vec2(), s.end.to_vec2(), *thickness, *colour));
                }
                Shape::Circle(centre, radius, colour) =>
                    graphics.draw_circle(centre.to_vec2(), *radius, *colour),
            }
        }
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2) {
        let new_position = Vector::new(
            position.x / self.scale_factor,
            position.y / self.scale_factor);
        if self.dragging {
            if let Some(old_position) = self.mouse_position {
                if !old_position.is_close(&new_position) && self.logging_count_left > 0 {
                    let start = self.frame.reverse_translate_to_unit(&old_position);
                    let end = self.frame.reverse_translate_to_unit(&new_position);
                    // println!("window_handler@85 {} {} {} {}", old_position, new_position, start, end);
                    let segment = Segment::new(start, end);
                    let drag = self.painter.drag_line(segment, self.pen_thickness, self.pen_colour);
                    if let Some(painter_response) = drag {
                        match painter_response {
                            PainterResponse::Add(shape_details) =>
                                self.shape_shifter.add_shape(shape_details),
                            PainterResponse::ColorChange(color) => {
                                self.pen_colour = color;
                            }
                            PainterResponse::ThicknessChange(thickness) => {
                                self.pen_thickness = thickness;
                            }
                        }
                    }
                    // if let Some(shape_details) = self.painter.drag_line(
                    //     segment, self.pen_thickness, self.pen_colour) {
                    //     self.shape_shifter.add_shape(shape_details);
                    // }
                    self.mouse_position = Some(new_position);
                    self.logging_count_left -= 1;
                    helper.request_redraw();
                }
            } else {
                self.mouse_position = Some(new_position);
            }
        }
    }

    fn on_mouse_button_down(&mut self, _helper: &mut WindowHelper, _button: MouseButton) {
        self.dragging = true;
        self.mouse_position = None;
    }

    fn on_mouse_button_up(&mut self, _helper: &mut WindowHelper, _button: MouseButton) {
        self.dragging = false;
    }
}


