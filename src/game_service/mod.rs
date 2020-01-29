mod game;
mod turn_validator;

use game::Game;
use game::Id;
use std::vec::Vec;

enum GameMessage {
    GameEvent(game::PlayerEvent),
    YourTurn,
    TurnAccepted(game::Turn),
    InvalidTurn,
    GameOver(Id),
}

pub trait PlayerChannel {
    /// посылка сообщений клиенту
    fn send(&mut self, msgs: Vec<GameMessage>);
}

// сервис игры следит за тем в какой последовательности ходят игроки
// а так же отслеживает правило того что игроки не могут использовать только одни удары
// без движений
struct GameService<C1, C2>
where
    C1: PlayerChannel,
    C2: PlayerChannel,
{
    channels: (C1, C2),
    game: Game,
}

impl<C1, C2> GameService<C1, C2>
where
    C1: PlayerChannel,
    C2: PlayerChannel,
{
    //создание нового сервиса игры
    fn new(one_player: C1, another_player: C2) -> Self
    where
        C1: PlayerChannel,
        C2: PlayerChannel,
    {
        GameService {
            channels: (one_player, another_player),
            game: Game::create_standart(),
        }
    }

    //стартуем игру
    fn start() {}
    // очередной игрок сделал ход
    fn turn(&mut self, player_id: Id, turn: game::Turn) {}
}
