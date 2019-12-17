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
    frame_idx: usize,
    next_action_in_parallel_flag: bool
}

impl Game {
    pub fn new( field: Field, first_player: Player, second_player: Player ) -> Game {
        Game {
            field,
            players: [ first_player, second_player ],
            events: Vec::new(),
            frame_idx: 0,
            next_action_in_parallel_flag: false
        }
    }
}

fn make_turn<S>( state: &mut S, player: Id, turn: Turn ) where S : GameState + EventsBuilder {
    match turn {
        Turn::MoveForward => turns::move_forward( state, player ),
        Turn::MoveBackward => unimplemented!(),
        Turn::MoveLeft => unimplemented!(),
        Turn::MoveRight => unimplemented!(),
        Turn::Pass => turns::pass( state, player ),
        Turn::HitStraightLeft => unimplemented!(),
        Turn::HitStraightRight => unimplemented!(),
        Turn::HitHookLeft => unimplemented!(),
        Turn::HitHookRight => unimplemented!()
    }
}