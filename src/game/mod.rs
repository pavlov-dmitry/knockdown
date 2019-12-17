pub mod state;
mod field;
mod game_event;
mod player;
mod turn;
mod types;
mod turns;

use state::GameState;
use types::{ Id, Point, Angle };
use turn::Turn;
use game_event::{ GameEvent, EventsBuilder };
use field::Field;
use player::Player;

#[derive(Debug)]
struct Game {
    field: Field,
    players: [Player; 2],
    events: Vec<GameEvent>,
    frame_idx: usize
}

impl Game {
    pub fn new( field: Field, first_player: Player, second_player: Player ) -> Game {
        Game {
            field,
            players: [ first_player, second_player ],
            events: Vec::new(),
            frame_idx: 0
        }
    }
}

fn make_turn<S>( state: &mut S, player: Id, turn: Turn ) where S : GameState + EventsBuilder {
    match turn {
        Turn::MoveForward => unimplemented!(),//turns::move_forward( &mut state, player ),
        Turn::MoveBackward => unimplemented!(),
        Turn::MoveLeft => unimplemented!(),
        Turn::MoveRight => unimplemented!(),
        Turn::Pass => unimplemented!(),
        Turn::HitStraightLeft => unimplemented!(),
        Turn::HitStraightRight => unimplemented!(),
        Turn::HitHookLeft => unimplemented!(),
        Turn::HitHookRight => unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_forward() {
        let first_player = Player::new( 0, 3, Point::new( 1.0, 1.0 ), Angle::new( 0.0 ) );
        let second_player = Player::new( 1, 3, Point::new( 1.0, 4.0 ), Angle::new( 180.0 ) );
        let mut game = Game::new(
            Field::new( 5.0, 5.0 ),
            first_player.clone(),
            second_player.clone() 
        );
        make_turn( &mut game, 0, Turn::MoveForward );
        assert_eq!( game.events.len(), 1 );
        assert_eq!( game.frame_idx, 1 );
        assert_eq!( second_player, game.players[ 1 ] );
        assert_eq!( 
            first_player.clone(), 
            Player{ position: Point::new( 1.0, 2.5 ), ..first_player }
        );
    }
}