use super::GameState;
use super::EventsBuilder;
use super::types::{ Id };

pub fn pass<S: GameState + EventsBuilder>( game: &mut S, player_id: Id ) {
    let player = game.get_player( player_id );
    let target = game.get_player_target( player_id );
    let angle = player.position.angle_to( &target.position );
    if angle != player.angle {
        game.player_rotate( player_id, angle );
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::*;

    #[test]
    fn pass() {
        let player_id = 0;
        let target_id = 1;
        let first_player = Player::new( 0, 3, Point::new( 1.0, 1.0 ), Angle::new( 90.0 ) );
        let second_player = Player::new( 1, 3, Point::new( 1.0, 4.0 ), Angle::new( 180.0 ) );
        let mut game = Game::new(
            Field::new( 5.0, 5.0 ),
            first_player.clone(),
            second_player.clone() 
        );
        super::pass( &mut game, player_id );
        assert_eq!( game.events.len(), 1 );
        assert_eq!( game.frame_idx, 1 );
        assert_eq!( second_player, game.players[ target_id ] );
        assert_eq!( 
            game.players[ player_id ],
            Player {
                angle: Angle::new( 0.0 ),
                ..first_player
            }
        );
    }
}