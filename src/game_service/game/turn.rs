/// Варианты хода которые может совершить игрок
#[derive(Debug)]
pub enum Turn {
    /// пропуск хода
    Pass,
    /// движение вперёд
    MoveForward,
    /// движение назад
    MoveBackward,
    /// обойти слева
    MoveLeft,
    /// обойти справа
    MoveRight,
    /// прямой удар правой рукой
    HitStraightRight,
    /// прямой удар левой рукой
    HitStraightLeft,
    /// удар хуком правой рукой
    HitHookRight,
    /// удар хуком левой рукой
    HitHookLeft,
}
