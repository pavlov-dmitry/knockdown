use super::types::{Id, Point, BODY_SIZE, HALF_BODY_SIZE};
use super::EventsBuilder;
use super::GameState;

pub fn hit_straight_left<G: GameState + EventsBuilder>(game: &mut G, player_id: Id) {
    game.player_hit_straight_left(player_id);
    hit_turn(game, player_id, is_hit_straight_left);
}

pub fn hit_straight_right<G: GameState + EventsBuilder>(game: &mut G, player_id: Id) {
    game.player_hit_straight_right(player_id);
    hit_turn(game, player_id, is_hit_straight_right);
}

pub fn hit_hook_left<G: GameState + EventsBuilder>(game: &mut G, player_id: Id) {
    game.player_hit_hook_left(player_id);
    hit_turn(game, player_id, is_hit_hook_left);
}

pub fn hit_hook_right<G: GameState + EventsBuilder>(game: &mut G, player_id: Id) {
    game.player_hit_hook_right(player_id);
    hit_turn(game, player_id, is_hit_hook_right);
}

fn hit_turn<G, F>(game: &mut G, player_id: Id, hit_check: F)
where
    G: GameState + EventsBuilder,
    F: Fn(&Point, &Angle, &Point) -> bool,
{
    let player = game.get_player(player_id);
    let target = game.get_player_target(player_id);
    super::turn_on_target_if_need(game, player_id, false);
    if hit_check(player.position, player.angle, target.position) {
        let target_id = game.get_player_target_id(player_id);
        let angle = target.position.angle_to(player.position).opposite();
        let new_target_position = target.position.layout_point(angle, BODY_SIZE * 2);
        game.in_same_time()
            .set_player_beaten(target_id)
            .in_same_time()
            .player_move_by_line_to(new_target_position);
        super::turn_on_target_if_need(game, target_id, true);
    }
}

fn is_hit_straight_left(player_pos: &Point, player_angle: Angle, target_pos: &Player) -> bool {
    unimplemented!()
}

fn is_hit_straight_right(player_pos: &Point, player_angle: Angle, target_pos: &Player) -> bool {
    unimplemented!()
}

fn is_hit_hook_left(player_pos: &Point, player_angle: Angle, target_pos: &Player) -> bool {
    unimplemented!()
}

fn is_hit_hook_right(player_pos: &Point, player_angle: Angle, target_pos: &Player) -> bool {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_hit() {}
}
