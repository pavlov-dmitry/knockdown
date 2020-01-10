use super::types::Id;
use super::EventsBuilder;
use super::GameState;

pub fn pass<S: GameState + EventsBuilder>(game: &mut S, player_id: Id) {
    super::turn_on_target_if_need(game, player_id, false);
}

#[cfg(test)]
mod tests {
    use super::super::super::*;

    #[test]
    fn pass() {
        let player_id = 0;
        let target_id = 1;
        let first_player = Player::new(0, 3, Point::new(1.0, 1.0), Angle::new(90.0));
        let second_player = Player::new(1, 3, Point::new(1.0, 4.0), Angle::new(180.0));
        let mut game = Game::new(
            Field::new(5.0, 5.0),
            first_player.clone(),
            second_player.clone(),
        );
        super::pass(&mut game, player_id);
        assert_eq!(game.events.len(), 1);
        assert_eq!(game.frame_idx, 1);
        assert_eq!(second_player, game.players[target_id]);
        assert_eq!(
            game.players[player_id],
            Player {
                angle: Angle::new(0.0),
                ..first_player
            }
        );
    }

    #[test]
    fn pass_no_rotate() {
        let player_id = 0;
        let target_id = 1;
        let first_player = Player::new(0, 3, Point::new(1.0, 1.0), Angle::new(0.0));
        let second_player = Player::new(1, 3, Point::new(1.0, 4.0), Angle::new(180.0));
        let mut game = Game::new(
            Field::new(5.0, 5.0),
            first_player.clone(),
            second_player.clone(),
        );
        super::pass(&mut game, player_id);
        assert_eq!(game.events.len(), 0);
        assert_eq!(game.frame_idx, 0);
        assert_eq!(second_player, game.players[target_id]);
        assert_eq!(game.players[player_id], first_player);
    }
}
