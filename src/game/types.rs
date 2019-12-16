pub type Duration = f32;
pub type Id = usize;
pub type Degree = f32;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    pub fn new( x: f32, y: f32 ) -> Self {
        Point {
            x,
            y
        }
    }
}