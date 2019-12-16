#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub width: f32,
    pub height: f32
}

impl Field {
    pub fn new( width: f32, height: f32 ) -> Field {
        Field {
            width,
            height
        }
    }
}