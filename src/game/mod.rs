pub mod state;
mod field;
mod game_event;
mod player;
mod turn;
mod types;

use state::GameState;
use types::{ Id, Point };
use turn::Turn;
use game_event::GameEvent;
use field::Field;
use player::Player;

fn make_turn( state: &mut GameState, player: Id, turn: Turn ) -> Vec<GameEvent> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_forward() {
        let mut state = GameState::new(
            Field::new( 5.0, 5.0 ),
            Player::new( 0, 3, Point::new( 1.0, 1.0 ), 0.0 ),
            Player::new( 1, 3, Point::new( 1.0, 4.0 ), 180.0 )
        );
        let expected_state = GameState::new(
            state.field.clone(),
            Player::new( 0, 3, Point::new( 1.0, 2.5 ), 0.0 ),
            state.players[ 1 ].clone()
        );
        let events = make_turn( &mut state, 0, Turn::MoveForward );
        assert_eq!( events.len(), 1 );
        assert_eq!( state, expected_state );
    }
}