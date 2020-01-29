use super::player::Player;
use super::turn::Turn;
use super::types::{Angle, Duration, HitPoints, Id, Point};
use super::Game;
use std::default::Default;

/// Действие произошедшее с игроком.
/// Изменения могут происходить паралельно.
/// Всё время в игре тактируетс по игровым фреймам, и несколько изменений могут произойти
/// в одном и том же временном фрейме (time_frame_idx).
#[derive(Debug, Clone)]
pub struct PlayerEvent {
    /// идентификатор игрока
    player_id: Id,
    /// изменение которое произошло с игроком
    action: PlayerDiff,
    /// индекс временного окна этого изменения
    time_frame_idx: usize,
}

/// Изменения которые произошли с игроком
#[derive(Debug, Clone)]
enum PlayerDiff {
    MoveTo(Point),
    MoveByCircle(MoveByCircle),
    RotateTo(Angle),
    StraightHitLeft,
    StraightHitRight,
    HookHitLeft,
    HookHitRight,
    /// переход в состояние побитого
    BeatenState,
    /// изменение в кол-ве жизней
    HitPoints(HitPoints),
}

#[derive(Debug, Clone)]
struct MoveByCircle {
    rotation_point: Point,
    angle_diff: f32,
}

#[derive(Debug, Clone)]
struct GameOver {
    winner_player_id: Id,
}

/// Типаж изменения состояния игры
pub trait EventsBuilder {
    /// говорит что следующее действие происходит в то же время что и предыдущее
    fn in_same_time(&mut self) -> &mut Self;
    /// повернуть игрока так чтобы он смотрел по определённому углу
    fn player_rotate(&mut self, player_id: Id, azimuth: Angle) -> &mut Self;
    /// игрок передвигается по прямой в определённую точку
    fn player_move_by_line_to(&mut self, player_id: Id, pos: Point) -> &mut Self;
    /// игрок передвигается по кругу
    fn player_move_by_circle(
        &mut self,
        player_id: Id,
        rotation_point: Point,
        angle_diff: f32,
    ) -> &mut Self;

    /// игрок ударил прямым слева
    fn player_hit_straight_left(&mut self, player_id: Id) -> &mut Self;
    /// игрок ударил прямым справа
    fn player_hit_straight_right(&mut self, player_id: Id) -> &mut Self;
    /// игрок ударил хуком слева
    fn player_hit_hook_left(&mut self, player_id: Id) -> &mut Self;
    /// игрок ударил хуком справа
    fn player_hit_hook_right(&mut self, player_id: Id) -> &mut Self;
    /// переводит игрока в состояние побитого
    fn set_player_beaten(&mut self, player_id: Id) -> &mut Self;
    /// устанавливаем кол-во жизней игрока
    fn set_player_hitpoints(&mut self, player_id: Id, hitpoint: HitPoints) -> &mut Self;
    /// конец игры
    fn game_over(&mut self, winner_id: Id) -> &mut Self;
}

impl EventsBuilder for Game {
    fn in_same_time(&mut self) -> &mut Self {
        self.next_action_in_parallel_flag = true;
        self
    }

    fn player_rotate(&mut self, player_id: Id, azimuth: Angle) -> &mut Self {
        self.add_player_event(player_id, PlayerDiff::RotateTo(azimuth.clone()));
        self.players[player_id].angle = azimuth;
        self
    }

    fn player_move_by_line_to(&mut self, player_id: Id, pos: Point) -> &mut Self {
        self.add_player_event(player_id, PlayerDiff::MoveTo(pos.clone()));
        self.players[player_id].position = pos;
        self
    }

    fn player_move_by_circle(
        &mut self,
        player_id: Id,
        rotation_point: Point,
        angle_diff: f32,
    ) -> &mut Self {
        self.add_player_event(
            player_id,
            PlayerDiff::MoveByCircle(MoveByCircle {
                rotation_point: rotation_point.clone(),
                angle_diff,
            }),
        );
        let player = &mut self.players[player_id];
        let angle_on_player = rotation_point.angle_to(&player.position);
        let angle_on_new_player_position = angle_on_player + angle_diff;
        let distance_to_player = rotation_point.distance_to(&player.position);
        player.position =
            rotation_point.layout_point(&angle_on_new_player_position, distance_to_player);
        self
    }

    fn set_player_beaten(&mut self, player_id: Id) -> &mut Self {
        self.add_player_event(player_id, PlayerDiff::BeatenState)
    }

    fn player_hit_straight_left(&mut self, player_id: Id) -> &mut Self {
        self.add_player_event(player_id, PlayerDiff::StraightHitLeft)
    }

    fn player_hit_straight_right(&mut self, player_id: Id) -> &mut Self {
        self.add_player_event(player_id, PlayerDiff::StraightHitRight)
    }

    fn player_hit_hook_left(&mut self, player_id: Id) -> &mut Self {
        self.add_player_event(player_id, PlayerDiff::HookHitLeft)
    }

    fn player_hit_hook_right(&mut self, player_id: Id) -> &mut Self {
        self.add_player_event(player_id, PlayerDiff::HookHitRight)
    }

    fn set_player_hitpoints(&mut self, player_id: Id, hitpoints: HitPoints) -> &mut Self {
        self.players[player_id].hit_points = hitpoints;
        self.add_player_event(player_id, PlayerDiff::HitPoints(hitpoints))
    }

    fn game_over(&mut self, winner_id: Id) -> &mut Self {
        self.winner = Some(winner_id);
        self
    }
}

impl Game {
    fn add_player_event(&mut self, player_id: Id, action: PlayerDiff) -> &mut Self {
        if !self.next_action_in_parallel_flag {
            self.frame_idx += 1;
        }
        self.next_action_in_parallel_flag = false;
        self.events.push(PlayerEvent {
            player_id,
            action,
            time_frame_idx: self.frame_idx,
        });
        self
    }
}
