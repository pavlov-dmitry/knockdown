mod field;
mod game_event;
mod player;
pub mod state;
mod turn;
mod turns;
mod types;

use field::Field;
use game_event::{EventsBuilder, GameEvent};
use player::Player;
use state::GameState;
use turn::Turn;
use types::{Angle, Id, Point};

#[derive(Debug)]
struct Game {
    field: Field,
    players: [Player; 2],
    events: Vec<GameEvent>,
    frame_idx: usize,
    next_action_in_parallel_flag: bool,
    game_over: bool,
}

impl Game {
    pub fn new(field: Field, first_player: Player, second_player: Player) -> Game {
        Game {
            field,
            players: [first_player, second_player],
            events: Vec::new(),
            frame_idx: 0,
            next_action_in_parallel_flag: false,
            game_over: false,
        }
    }
}

fn make_turn<G>(game: &mut G, player: Id, turn: Turn)
where
    G: GameState + EventsBuilder,
{
    match turn {
        Turn::MoveForward => turns::move_forward(game, player),
        Turn::MoveBackward => turns::move_backward(game, player),
        Turn::MoveLeft => turns::move_left(game, player),
        Turn::MoveRight => turns::move_right(game, player),
        Turn::Pass => turns::pass(game, player),
        Turn::HitStraightLeft => unimplemented!(),
        Turn::HitStraightRight => unimplemented!(),
        Turn::HitHookLeft => unimplemented!(),
        Turn::HitHookRight => unimplemented!(),
    }
    turns::back_to_the_ring(game, 0);
    turns::back_to_the_ring(game, 1);
    check_on_knockout(game, 0);
    check_on_knockout(game, 1);
}

fn check_on_knockout<G>(game: &mut G, player_id: Id)
where
    G: GameState + EventsBuilder,
{
    let player = game.get_player(player_id);
    if player.hit_points == 0 {
        game.game_over(game.get_player_target_id(player_id));
    }
}
