use super::types::{self, Id};
use super::EventsBuilder;
use super::GameState;

mod move_forward;
pub use move_forward::move_forward;

mod pass;
pub use pass::pass;

mod move_backward;
pub use move_backward::move_backward;

mod move_by_circle;
pub use move_by_circle::move_left;
pub use move_by_circle::move_right;

mod back_to_the_ring;
pub use back_to_the_ring::back_to_the_ring;

pub fn turn_on_target_if_need<S: GameState + EventsBuilder>(
    game: &mut S,
    player_id: Id,
    in_same_time: bool,
) {
    let player = game.get_player(player_id);
    let target = game.get_player_target(player_id);
    let angle_on_target = player.position.angle_to(&target.position);
    if player.angle != angle_on_target {
        if (in_same_time) {
            game.in_same_time();
        }
        game.player_rotate(player_id, angle_on_target);
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn rotate_on_target() {
        let first_player = Player::new(0, 3, Point::new(1.0, 1.0), Angle::new(90.0));
        let second_player = Player::new(1, 3, Point::new(1.0, 4.0), Angle::new(180.0));
        let mut game = Game::new(
            Field::new(5.0, 5.0),
            first_player.clone(),
            second_player.clone(),
        );
        super::turn_on_target_if_need(&mut game, 0, false);
        assert_eq!(game.events.len(), 1);
        assert_eq!(game.frame_idx, 1);
        assert_eq!(second_player, game.players[1]);
        assert_eq!(
            game.players[0],
            Player {
                angle: Angle::new(0.0),
                ..first_player
            }
        );
    }
    #[test]
    fn no_need_rotate_on_target() {
        let first_player = Player::new(0, 3, Point::new(1.0, 1.0), Angle::new(0.0));
        let second_player = Player::new(1, 3, Point::new(1.0, 4.0), Angle::new(180.0));
        let mut game = Game::new(
            Field::new(5.0, 5.0),
            first_player.clone(),
            second_player.clone(),
        );
        let first_player = Player {
            angle: Angle::new(0.0),
            ..first_player
        };
        super::turn_on_target_if_need(&mut game, 0, false);
        assert_eq!(game.events.len(), 0);
        assert_eq!(game.frame_idx, 0);
        assert_eq!(second_player, game.players[1]);
        assert_eq!(first_player, game.players[0]);
    }
}
