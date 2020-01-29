mod field;
mod game_event;
mod player;
pub mod state;
mod turn;
mod turns;
mod types;

use field::Field;
use game_event::EventsBuilder;
pub use game_event::PlayerEvent;
use player::Player;
use state::GameState;
pub use turn::Turn;
pub use types::Id;
use types::{Angle, Point, BODY_SIZE};

#[derive(Debug)]
pub struct Game {
    field: Field,
    players: [Player; 2],
    events: Vec<PlayerEvent>,
    frame_idx: usize,
    next_action_in_parallel_flag: bool,
    winner: Option<Id>,
}

impl Game {
    pub fn create_standart() -> Game {
        let field_size = 5.0 * BODY_SIZE;
        let center_of_ring = Point::new(field_size / 2.0, field_size / 2.0);
        let first_position = center_of_ring.layout_point(&Angle::new(-45.0), BODY_SIZE * 1.25);
        let second_positon = center_of_ring.layout_point(&Angle::new(135.0), BODY_SIZE * 1.25);
        let hp = 3;
        Game {
            field: Field::new(field_size, field_size),
            players: [
                Player::new(hp, first_position, Angle::new(135.0)),
                Player::new(hp, second_positon, Angle::new(-45.0)),
            ],
            events: Vec::new(),
            frame_idx: 0,
            next_action_in_parallel_flag: false,
            winner: None,
        }
    }

    pub fn new(field: Field, first_player: Player, second_player: Player) -> Game {
        Game {
            field,
            players: [first_player, second_player],
            events: Vec::new(),
            frame_idx: 0,
            next_action_in_parallel_flag: false,
            winner: None,
        }
    }

    pub fn make_turn(&mut self, player_id: Id, turn: Turn) -> Vec<PlayerEvent> {
        make_turn(self, player_id, turn);
        let result = self.events.clone();
        self.events.clear();
        result
    }

    pub fn winner(&self) -> Option<Id> {
        self.winner.clone()
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
        Turn::HitStraightLeft => turns::hit_straight_left(game, player),
        Turn::HitStraightRight => turns::hit_straight_right(game, player),
        Turn::HitHookLeft => turns::hit_hook_left(game, player),
        Turn::HitHookRight => turns::hit_hook_right(game, player),
    }
    const FIRST_PLAYER_ID: Id = 0;
    const SECOND_PLAYER_ID: Id = 1;
    turns::back_to_the_ring(game, FIRST_PLAYER_ID);
    turns::back_to_the_ring(game, SECOND_PLAYER_ID);
    check_on_knockout(game, FIRST_PLAYER_ID);
    check_on_knockout(game, SECOND_PLAYER_ID);
}

fn check_on_knockout<G>(game: &mut G, player_id: Id)
where
    G: GameState + EventsBuilder,
{
    let player = game.get_player(player_id);
    if player.hit_points <= 0 {
        game.game_over(game.get_player_target_id(player_id));
    }
}
