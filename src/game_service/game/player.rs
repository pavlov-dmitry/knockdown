use super::types::{Angle, HitPoints, Id, Point};

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub hit_points: HitPoints,
    pub max_hit_points: HitPoints,
    pub position: Point,
    pub angle: Angle,
}

impl Player {
    pub fn new(hit_points: u8, position: Point, angle: Angle) -> Player {
        Player {
            hit_points,
            max_hit_points: hit_points,
            position,
            angle,
        }
    }
}
