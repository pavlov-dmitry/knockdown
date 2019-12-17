use super::GameState;
use super::EventsBuilder;
use super::types::{ Id };

pub fn move_forward<S: GameState + EventsBuilder>( game: &mut S, player_id: Id ) {
    let player = game.get_player( player_id );
    let target = game.get_player_target( player_id );
    let angle = player.position.angle_to( &target.position );
    let distance = player.position.distance_to( &target.position );
    let distance = distance.min( 1.0 );
    let new_player_pos = player.position.layout_point( &angle, distance );
    if angle != player.angle {
        game.player_rotate(player_id, angle);
    }
    game.player_move_by_line_to( player_id, new_player_pos );
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::super::*;

    #[test]
    fn move_forward() {
        let player_id = 0;
        let first_player = Player::new( 0, 3, Point::new( 1.0, 1.0 ), Angle::new( 90.0 ) );
        let second_player = Player::new( 1, 3, Point::new( 1.0, 4.0 ), Angle::new( 180.0 ) );
        let mut game = Game::new(
            Field::new( 5.0, 5.0 ),
            first_player.clone(),
            second_player.clone() 
        );
        super::move_forward( &mut game, player_id );
        assert_eq!( game.events.len(), 2 );
        assert_eq!( game.frame_idx, 2 );
        assert_eq!( second_player, game.players[ 1 ] );
        assert_eq!( 
            game.players[ player_id ],
            Player { 
                position: Point::new( 1.0, 2.0 ),
                angle: Angle::new( 0.0 ),
                ..first_player 
            }
        );
    }
}