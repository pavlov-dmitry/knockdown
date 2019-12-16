use super::field::{ Field };
use super::player::{ Player };

#[derive(PartialEq, Debug)]
pub struct GameState {
    pub field: Field,
    pub players: [Player; 2]
}

impl GameState {
    pub fn new( field: Field, first_player: Player, second_player: Player ) -> GameState {
        GameState {
            field,
            players: [ first_player, second_player ]
        }
    }
}