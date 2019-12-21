pub type Duration = f32;
pub type Distance = f32;
pub type Id = usize;

pub const BODY_SIZE: Distance = 1.0;
pub const HALF_BODY_SIZE: Distance = BODY_SIZE / 2.0;

mod angle;
mod point;

pub type Angle = angle::Angle;
pub type Point = point::Point;
