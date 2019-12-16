use super::types::{ Id, Point, Degree };

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    user_id: Id,
    hit_points: u8,
    position: Point,
    angle: Degree
}

impl Player {
    pub fn new( user_id: Id, hit_points: u8, position: Point, angle: Degree ) -> Player {
        Player {
            user_id,
            hit_points,
            position,
            angle
        }
    }

    pub fn change_pos( &mut self, new_position: Point ) {
        self.position = new_position;
    }

    pub fn change_angle( &mut self, new_angle: Degree ) {
        self.angle = new_angle;
    }
}