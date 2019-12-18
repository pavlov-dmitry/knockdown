use super::GameState;
use super::EventsBuilder;
use super::types::{ Id, HALF_BODY_SIZE };

pub fn move_backward<S: GameState + EventsBuilder>( game: &mut S, player_id: Id ) {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    #[test]
    fn move_backward() {
        let player_id = 0;
        let first_player = Player::new( 0, 3, Point::new( 1.0, 1.0 ), Angle::new( 90.0 ) );
        let second_player = Player::new( 1, 3, Point::new( 1.0, 4.0 ), Angle::new( 180.0 ) );
        let mut game = Game::new(
            Field::new( 5.0, 5.0 ),
            first_player.clone(),
            second_player.clone() 
        );
        super::move_backward( &mut game, player_id );
        assert_eq!( game.events.len(), 2 );
        assert_eq!( game.frame_idx, 2 );
        assert_eq!( second_player, game.players[ 1 ] );
        assert_eq!( 
            game.players[ player_id ],
            Player { 
                position: Point::new( 1.0, 0.5 ),
                angle: Angle::new( 0.0 ),
                ..first_player 
            }
        );
    }

    #[test]
    fn move_backward_no_rotate() {
        let player_id = 0;
        let first_player = Player::new( 0, 3, Point::new( 1.0, 1.0 ), Angle::new( 0.0 ) );
        let second_player = Player::new( 1, 3, Point::new( 1.0, 4.0 ), Angle::new( 180.0 ) );
        let mut game = Game::new(
            Field::new( 5.0, 5.0 ),
            first_player.clone(),
            second_player.clone() 
        );
        super::move_backward( &mut game, player_id );
        assert_eq!( game.events.len(), 1 );
        assert_eq!( game.frame_idx, 1 );
        assert_eq!( second_player, game.players[ 1 ] );
        assert_eq!( 
            game.players[ player_id ],
            Player { 
                position: Point::new( 1.0, 0.5 ),
                ..first_player 
            }
        );
    }
}