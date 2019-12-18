use super::GameState;
use super::EventsBuilder;
use super::types::{ Id, BODY_SIZE };

pub fn move_left<S: GameState + EventsBuilder>( game: &mut S, player_id: Id ) {
    move_by_circle(game, player_id, 60.0 );
}

pub fn move_right<S: GameState + EventsBuilder>( game: &mut S, player_id: Id ) {
    move_by_circle(game, player_id, -60.0 );
}

fn move_by_circle<S: GameState + EventsBuilder>( game: &mut S, player_id: Id, angle_diff : f32 ) {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    fn make_values() -> ( Player, Player, Game ) {
        let first_player = Player::new( 0, 3, Point::new( 1.0, 1.0 ), Angle::new( 90.0 ) );
        let second_player = Player::new( 1, 3, Point::new( 1.0, 4.0 ), Angle::new( 180.0 ) );
        let game = Game::new(
            Field::new( 5.0, 5.0 ),
            first_player.clone(),
            second_player.clone() 
        );
        (first_player, second_player, game)
    }

    #[test]
    fn move_left() {
        let player_id = 0;
        let (first_player, second_player, mut game) = make_values();
        super::move_left( &mut game, player_id );
        assert_eq!( game.events.len(), 3 );
        assert_eq!( game.frame_idx, 3 );
        assert_eq!( second_player, game.players[ 1 ] );
        let updated_player = game.get_player( player_id );
        assert_eq!( 
            updated_player.clone(),
            Player { 
                position: updated_player.position.clone(),
                angle: updated_player.angle.clone(),
                ..first_player 
            }
        );
        assert!( updated_player.position.x < first_player.position.x );
        let div_x = ( first_player.position.x - updated_player.position.x ).abs();
        assert!( BODY_SIZE < div_x );
        assert!( 
            updated_player.position.distance_to(&second_player.position)
             <
            first_player.position.distance_to(&second_player.position) 
        )
    }

    #[test]
    fn move_right() {
        let player_id = 0;
        let (first_player, second_player, mut game) = make_values();
        super::move_left( &mut game, player_id );
        assert_eq!( game.events.len(), 3 );
        assert_eq!( game.frame_idx, 3 );
        assert_eq!( second_player, game.players[ 1 ] );
        let updated_player = game.get_player( player_id );
        assert_eq!( 
            updated_player.clone(),
            Player { 
                position: updated_player.position.clone(),
                angle: updated_player.angle.clone(),
                ..first_player 
            }
        );
        assert!( first_player.position.x < updated_player.position.x );
        let div_x = ( first_player.position.x - updated_player.position.x ).abs();
        assert!( BODY_SIZE < div_x );
        assert!( 
            updated_player.position.distance_to(&second_player.position)
             <
            first_player.position.distance_to(&second_player.position) 
        )
    }
}