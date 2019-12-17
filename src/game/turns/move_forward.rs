use super::GameState;
use super::EventsBuilder;
use super::types::{ Id };

pub fn move_forward<S: GameState + EventsBuilder>( game: &mut S, player_id: Id ) {
    let player = game.get_player( player_id );
    let target = game.get_player_target( player_id );
    let angle = player.position.angle_to( &target.position );
    let distance = player.position.distance_to( &target.position );
    let distance = distance.min( 1.5 );
    let new_player_pos = player.position.layout_point( &angle, distance );
    if angle != player.angle {
        game.player_rotate(player_id, angle);
    }
    game.player_move_by_line_to( player_id, new_player_pos );
}