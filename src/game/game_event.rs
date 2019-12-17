use super::types::{ Id, Duration, Point, Angle };
use super::turn::Turn;
use super::player::Player;
use super::Game;
use std::default::Default;

/// Игровые событий
#[derive(Debug)]
pub enum GameEvent {
    /// что-то произошло с игроком
    PlayerEvent( PlayerEvent ), 
    /// событие по которому сервер ожидает хода игрока
    YourTurn, 
    /// подтверждение получения хода
    TurnAccepted( Turn ), 
    /// конец игры
    GameOver( GameOver ) 
}

/// Действие произошедшее с игроком.
/// Изменения могут происходить паралельно.
/// Всё время в игре тактируетс по игровым фреймам, и несколько изменений могут произойти
/// в одном и том же временном фрейме (time_frame_idx). Так же каждое действие может иметь
/// длительность внутри игрового фрейма(duration), и пауза перед началом изменения
/// внутри фрейма(pause_before_start)
/// Временные фреймы могут быть разными по длительности, это опеределяется сумарной длиной
/// всех событий внутри временного фрейма.
#[derive(Debug)]
pub struct PlayerEvent {
    /// идентификатор игрока
    player_id: Id, 
    /// изменение которое произошло с игроком
    action: PlayerDiff, 
    /// индекс временного окна этого изменения
    time_frame_idx: usize, 
    /// пауза перед началом этого изменения
    pause_before_start: Duration, 
    /// относительное время во временном окне
    duration: Duration, 
}

/// Изменения которые произошли с игроком
#[derive(Debug)]
enum PlayerDiff {
    MoveTo( Point ),
    MoveByCircle(MoveByCircle),
    RotateTo( Angle ),
    StraightHitLeft,
    StraightHitRight,
    HookHitLeft,
    HookHitRight,
    /// переход в состояние побитого
    BeatenState, 
    /// изменение в кол-ве жизней
    HitPoints( u8 ) 
}

#[derive(Debug)]
struct MoveByCircle {
    rotation_point: Point,
    angle: Angle
}

impl MoveByCircle {
    pub fn calc_end_point(&self) -> Point {
        unimplemented!();
    }
}

#[derive(Debug)]
struct GameOver {
    winner_player_id: Id
}

impl Default for PlayerEvent {
    fn default() -> Self {
        PlayerEvent {
            player_id: 0,
            action: PlayerDiff::BeatenState,
            time_frame_idx: 0,
            pause_before_start: 0.0,
            duration: 1.0
        }
    }
}

/// Типаж изменения состояния игры
pub trait EventsBuilder {
    /// повернуть игрока так чтобы он смотрел на опеределённую точку
    fn player_rotate( &mut self, player_id: Id, azimuth: Angle ) -> &mut Self;
    /// задать скорость изменения предыдущего действия
    fn with_speed( &mut self, speed: Duration ) -> &mut Self;
    /// задать паузу которую нужно выдержать в этом фрейме событий
    fn with_pause( &mut self, pause: Duration ) -> &mut Self;
}

impl EventsBuilder for Game {
    fn with_speed( &mut self, speed: Duration ) -> &mut Self {
        change_last_player_event( self, |e| e.duration = speed );
        self
    }

    fn with_pause( &mut self, pause: Duration ) -> &mut Self {
        change_last_player_event( self, |e| e.pause_before_start = pause );
        self
    }

    fn player_rotate( &mut self, player_id: Id, azimuth: Angle ) -> &mut Self {
        self.events.push( GameEvent::PlayerEvent( PlayerEvent {
            player_id,
            action: PlayerDiff::RotateTo( azimuth.clone() ),
            time_frame_idx: self.frame_idx,
            ..Default::default()
        }));
        self.players[ player_id ].angle = azimuth;
        self
    }
}

fn change_last_player_event<F>( game: &mut Game, f: F ) where F : Fn(&mut PlayerEvent) {
    if let Some( event ) = game.events.last_mut() {
        if let GameEvent::PlayerEvent( event ) = event {
            f( event );
        }
    }
}