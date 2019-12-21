use super::types::{Angle, Id, Point};

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub user_id: Id,
    pub hit_points: u8,
    pub position: Point,
    pub angle: Angle,
}

impl Player {
    pub fn new(user_id: Id, hit_points: u8, position: Point, angle: Angle) -> Player {
        Player {
            user_id,
            hit_points,
            position,
            angle,
        }
    }

    pub fn change_pos(&mut self, new_position: Point) {
        self.position = new_position;
    }

    pub fn change_angle(&mut self, new_angle: Angle) {
        self.angle = new_angle;
    }
}
