use super::GameState;
use super::EventsBuilder;
use super::super::types::{ Id };

pub fn move_forward<S: GameState + EventsBuilder>( state: &mut S, player_id: Id ) {
    let player = state.get_player( player_id );
    let target = state.get_player_target( player_id );
    let angle = player.position.angle_to( &target.position );
    let distance = player.position.distance_to( &target.position );
    let distance = distance.min( 1.5 );
    let new_player_pos = player.position.layout_point( &angle, distance );

    state
        .player_rotate(player_id, angle);
        //.player_line_move( player_id, new_player_pos );
}