use super::types::{Angle, Distance, Id, Point, BODY_SIZE, HALF_BODY_SIZE};
use super::EventsBuilder;
use super::GameState;

const GUARD_DISTANCE: Distance = HALF_BODY_SIZE / 3.0;
const STRAIGHT_LENGTH: Distance = BODY_SIZE * 1.5 - GUARD_DISTANCE;
const HOOK_LENGTH: Distance = BODY_SIZE - GUARD_DISTANCE;

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
    super::turn_on_target_if_need(game, player_id, false);
    let player = game.get_player(player_id);
    let target = game.get_player_target(player_id);
    if hit_check(&player.position, &player.angle, &target.position) {
        let target_id = game.get_player_target_id(player_id);
        // после получения удара, цель отлетает от игрока который её ударил
        let angle = target.position.angle_to(&player.position).opposite();
        let new_target_position = target.position.layout_point(&angle, BODY_SIZE * 2.0);

        let new_target_hitpoints = target.hit_points - 1;
        game.in_same_time()
            .set_player_beaten(target_id)
            .in_same_time()
            .set_player_hitpoints(target_id, new_target_hitpoints)
            .in_same_time()
            .player_move_by_line_to(target_id, new_target_position);
        super::turn_on_target_if_need(game, target_id, true);
    }
}

enum Hand {
    Left,
    Right,
}

fn is_hit_straight_left(player_pos: &Point, player_angle: &Angle, target_pos: &Point) -> bool {
    is_hit_straight(player_pos, player_angle, target_pos, Hand::Left)
}

fn is_hit_straight_right(player_pos: &Point, player_angle: &Angle, target_pos: &Point) -> bool {
    is_hit_straight(player_pos, player_angle, target_pos, Hand::Right)
}

fn is_hit_straight(
    player_pos: &Point,
    player_angle: &Angle,
    target_pos: &Point,
    hand: Hand,
) -> bool {
    // для прямого удара считаем просто попадание в прямоуголник перед игроком
    let pos = target_pos.translate_to(*player_angle, player_pos.clone());
    let (from_x, to_x) = match hand {
        Hand::Left => (-BODY_SIZE, HALF_BODY_SIZE),
        Hand::Right => (-HALF_BODY_SIZE, BODY_SIZE),
    };
    // две половинки тела это потому что позиция бойца это его центр, и надо их прибавить
    if HALF_BODY_SIZE < pos.y
        && pos.y < STRAIGHT_LENGTH + HALF_BODY_SIZE * 2.0
        && from_x < pos.x
        && pos.x < to_x
    {
        true
    } else {
        false
    }
}

fn is_hit_hook_left(player_pos: &Point, player_angle: &Angle, target_pos: &Point) -> bool {
    is_hit_hook(player_pos, player_angle, target_pos, Hand::Left)
}

fn is_hit_hook_right(player_pos: &Point, player_angle: &Angle, target_pos: &Point) -> bool {
    is_hit_hook(player_pos, player_angle, target_pos, Hand::Right)
}

fn is_hit_hook(player_pos: &Point, player_angle: &Angle, target_pos: &Point, hand: Hand) -> bool {
    // хук бьёт по кругу в одну из четверти круга
    let pos = target_pos.translate_to(*player_angle, player_pos.clone());
    let (from_x, to_x) = match hand {
        Hand::Left => (BODY_SIZE * -2.0, HALF_BODY_SIZE),
        Hand::Right => (-HALF_BODY_SIZE, BODY_SIZE * 2.0),
    };
    if -HALF_BODY_SIZE < pos.y && pos.y < BODY_SIZE * 2.0 && from_x < pos.x && pos.x < to_x {
        let distance = Point::new(0.0, 0.0).distance_to(&pos);
        // две половинки тела это потому что позиция бойца это его центр, и надо их прибавить
        if distance < HOOK_LENGTH + HALF_BODY_SIZE * 2.0 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_hit_straight() {
        let player_pos = Point::new(1.0, 1.0);
        let player_azimuth = Angle::new(90.0);
        assert!(is_hit_straight_left(
            &player_pos,
            &player_azimuth,
            &Point::new(3.0, 1.0)
        ));
        assert!(!is_hit_straight_left(
            &player_pos,
            &player_azimuth,
            &Point::new(3.5, 1.0)
        ));
        assert!(!is_hit_straight_left(
            &player_pos,
            &player_azimuth,
            &Point::new(2.0, 2.0)
        ));
        assert!(is_hit_straight_left(
            &player_pos,
            &player_azimuth,
            &Point::new(2.0, 1.7)
        ));
        assert!(!is_hit_straight_right(
            &player_pos,
            &player_azimuth,
            &Point::new(2.0, 1.7)
        ));
    }

    #[test]
    fn is_hit_hook() {
        let player_pos = Point::new(1.0, 1.0);
        let player_azimuth = Angle::new(90.0);
        assert!(!is_hit_hook_left(
            &player_pos,
            &player_azimuth,
            &Point::new(3.0, 1.0)
        ));
        assert!(is_hit_hook_left(
            &player_pos,
            &player_azimuth,
            &Point::new(2.5, 1.0)
        ));
        assert!(is_hit_hook_left(
            &player_pos,
            &player_azimuth,
            &Point::new(2.0, 2.0)
        ));
        assert!(!is_hit_hook_right(
            &player_pos,
            &player_azimuth,
            &Point::new(2.0, 2.0)
        ));
    }
}
