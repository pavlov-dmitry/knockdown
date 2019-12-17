use super::GameState;
use super::EventsBuilder;
use super::types::{ Id };

pub fn pass<S: GameState + EventsBuilder>( game: &mut S, player_id: Id ) {
    let player = game.get_player( player_id );
    let target = game.get_player_target( player_id );
    let angle = player.position.angle_to( &target.position );
    if angle != player.angle {
        game.player_rotate( player_id, angle );
    }
}