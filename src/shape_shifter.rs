use crate::frame::Frame;
use crate::shape::Shape;

pub struct ShapeShifter {
    shape_collections: Vec<Vec<Shape>>,
}

pub struct Rendering {
    index: usize,
    frame: Frame,
}

impl Rendering {
    pub fn new(index: usize, frame: Frame) -> Self {
        Self { index, frame }
    }
}

impl ShapeShifter {
    pub fn new() -> Self { Self { shape_collections: vec![] } }

    pub fn add(&mut self, shapes: Vec<Shape>) -> usize {
        self.shape_collections.push(shapes);
        let index = self.shape_collections.len() - 1;
        self.shape_collections[index].reserve(10000);
        index
    }

    pub fn render(&self, renderings: Vec<Rendering>, rendered_shapes: &mut Vec<Shape>) {
        renderings.iter().for_each(|detail|
            self.shape_collections[detail.index].iter().for_each(|shape|
                {
                    let shifted_shape = detail.frame.shift_shape(shape);
                    // println!("shape_shifter@33 {:?} \n                 {:?}", shape, shifted_shape);
                    rendered_shapes.push(shifted_shape);
                })
        );
    }

    pub fn add_shape(&mut self, add: AddShapeDetails) {
        // println!("shape_shifter@40 {} {:?}", add.index, add.shape);
        self.shape_collections[add.index].push(add.shape);
    }

    /*   pub fn add_shape22(&mut self, add: AddShapeDetails) {
           println!("shape_shifter@40 {} {:?}", add.index, add.shape);
           if let Some(shapes) = self.shape_collections.get_mut(add.index) {
               if !shapes.is_empty() {
                   if let Some(Shape::PolyLine(segment, _, _)) = shapes.last() {
                       if Some(last) = segment.last() {
                           todo!() // todo Check if the add shape is a Line or PolyLine. Append to last if continues
                       }
                   }
               } else {
                   shapes.push(add.shape);
               }
           }
       }
   */
}

#[derive(Debug, PartialEq, Clone)]
pub struct AddShapeDetails {
    index: usize,
    shape: Shape,
}

impl AddShapeDetails {
    pub fn new(index: usize, shape: Shape) -> Self {
        Self { index, shape }
    }
}
