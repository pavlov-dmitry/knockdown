use super::types::{ Id, Duration, Point, Degree };
use super::turn::Turn;

/// Игровые событий
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
enum PlayerDiff {
    MoveByLine(MoveByLine),
    MoveByCircle(MoveByCircle),
    Rotate(Rotate),
    StraightHitLeft,
    StraightHitRight,
    HookHitLeft,
    HookHitRight,
    /// переход в состояние побитого
    BeatenState, 
    /// изменение в кол-ве жизней
    HitPoints( u8 ) 
}

struct MoveByLine {
    from: Point,
    to: Point
}

struct MoveByCircle {
    from: Point,
    rotation_point: Point,
    angle: Degree
}

struct Rotate {
    from_angle: Degree,
    to_angle: Degree
}

struct GameOver {
    winner_player_id: Id
}
