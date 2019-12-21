use super::GameState;
use super::EventsBuilder;
use super::types::{ Id, HALF_BODY_SIZE };

pub fn back_to_the_ring<G: GameState + EventsBuilder>( game: &mut G, player_id: Id ) {
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;
    use super::super::types::{ HALF_BODY_SIZE, BODY_SIZE, Point };
    
    fn make_game( first_player: Player ) -> (Player, Game) {
        let second_player = Player::new( 1, 3, Point::new( 1.0, 4.5 ), Angle::new( 180.0 ) );
        let game = Game::new(
            Field::new( 5.0, 5.0 ),
            first_player,
            second_player.clone() 
        );
        (second_player, game)
    }

    #[test]
    fn no_return() {
        let first_player = Player::new( 0, 3, Point::new( 1.0, 1.0 ), Angle::new( 0.0 ) );
        let (second_player, mut game) = make_game( first_player.clone() );
        super::back_to_the_ring( &mut game, 0 );
        assert_eq!( game.events.len(), 0 );
        assert_eq!( game.frame_idx, 0 );
        assert_eq!( game.players[ 0 ], first_player );
        assert_eq!( game.players[ 1 ], second_player );
    }

    fn check_simple_return( first_player: Player ) {
        let (second_player, mut game) = make_game( first_player.clone() );
        super::back_to_the_ring( &mut game, 0 );
        assert_eq!( game.events.len(), 2 );
        assert_eq!( game.frame_idx, 1 );
        let player = game.get_player( 0 );
        let field = game.get_field();
        assert!( HALF_BODY_SIZE <= player.position.x && player.position.x <= field.width - HALF_BODY_SIZE );
        assert!( HALF_BODY_SIZE <= player.position.y && player.position.y <= field.height - HALF_BODY_SIZE );
        assert_eq!( game.players[ 1 ], second_player );
    }

    #[test]
    fn simple_return() {
        check_simple_return(Player::new( 0, 3, Point::new( -1.0, 1.0 ), Angle::new( 90.0 ) ));
        check_simple_return(Player::new( 0, 3, Point::new( 1.0, -1.0 ), Angle::new( 90.0 ) ));
        check_simple_return(Player::new( 0, 3, Point::new( 7.0, 1.0 ), Angle::new( 90.0 ) ));
        check_simple_return(Player::new( 0, 3, Point::new( 1.0, 9.0 ), Angle::new( 90.0 ) ));
        check_simple_return(Player::new( 0, 3, Point::new( -1.0, -1.0 ), Angle::new( 90.0 ) ));
        check_simple_return(Player::new( 0, 3, Point::new( -1.0, 7.0 ), Angle::new( 90.0 ) ));
        check_simple_return(Player::new( 0, 3, Point::new( 8.0, 7.0 ), Angle::new( 90.0 ) ));
        check_simple_return(Player::new( 0, 3, Point::new( 8.0, -7.0 ), Angle::new( 90.0 ) ));
    }

    #[test]
    fn return_and_move_target() {
        let first_player = Player::new( 0, 3, Point::new( 1.0, 6.0 ), Angle::new( 90.0 ) );
        let (second_player, mut game) = make_game( first_player.clone() );
        super::back_to_the_ring( &mut game, 0 );
        assert_eq!( game.events.len(), 3 );
        assert_eq!( game.frame_idx, 1 );
        assert_eq!( game.players[ 0 ], Player {
            position: Point::new( 1.0, 5.0 - HALF_BODY_SIZE ),
            ..first_player
        });
        assert_ne!( game.players[ 1 ].position, second_player.position );
        let distance = game.players[ 0 ].position.distance_to( &game.players[ 1 ].position );
        let distance_diff = (distance - BODY_SIZE ).abs();
        assert!( distance_diff < std::f32::EPSILON );
    }
}