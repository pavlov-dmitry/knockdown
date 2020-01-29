use super::game::Turn;

pub struct TurnValidator {
    forbidden_turns: Vec<Turn>,
}

impl TurnValidator {
    pub fn new() -> Self {
        TurnValidator {
            forbidden_turns: Vec::new(),
        }
    }

    pub fn validate_turn_and_remember(&mut self, turn: Turn) -> bool {
        let result = self.forbidden_turns.iter().find(|&&t| t == turn).is_none();
        if result == true {
            match turn {
                Turn::HitHookLeft
                | Turn::HitHookRight
                | Turn::HitStraightLeft
                | Turn::HitStraightRight => self.forbidden_turns.push(turn),
                Turn::MoveBackward
                | Turn::MoveForward
                | Turn::MoveLeft
                | Turn::MoveRight
                | Turn::Pass => self.forbidden_turns.clear(),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_strike() {
        let mut validator = TurnValidator::new();
        assert!(validator.validate_turn_and_remember(Turn::HitHookLeft));
        assert!(validator.validate_turn_and_remember(Turn::HitHookLeft) == false);
        assert!(validator.validate_turn_and_remember(Turn::HitHookRight));
        assert!(validator.validate_turn_and_remember(Turn::HitHookRight) == false);
        assert!(validator.validate_turn_and_remember(Turn::HitStraightLeft));
        assert!(validator.validate_turn_and_remember(Turn::HitStraightLeft) == false);
        assert!(validator.validate_turn_and_remember(Turn::HitStraightRight));
        assert!(validator.validate_turn_and_remember(Turn::HitStraightRight) == false);

        assert!(validator.validate_turn_and_remember(Turn::MoveLeft));
        assert!(validator.validate_turn_and_remember(Turn::MoveLeft));

        assert!(validator.validate_turn_and_remember(Turn::MoveRight));
        assert!(validator.validate_turn_and_remember(Turn::MoveRight));

        assert!(validator.validate_turn_and_remember(Turn::MoveForward));
        assert!(validator.validate_turn_and_remember(Turn::MoveForward));

        assert!(validator.validate_turn_and_remember(Turn::MoveBackward));
        assert!(validator.validate_turn_and_remember(Turn::MoveBackward));

        assert!(validator.validate_turn_and_remember(Turn::Pass));
        assert!(validator.validate_turn_and_remember(Turn::Pass));
    }

    #[test]
    fn reset_after_move() {
        let mut validator = TurnValidator::new();
        assert!(validator.validate_turn_and_remember(Turn::HitHookLeft));
        assert!(validator.validate_turn_and_remember(Turn::MoveRight));
        assert!(validator.validate_turn_and_remember(Turn::HitHookLeft));

        assert!(validator.validate_turn_and_remember(Turn::MoveLeft));
        assert!(validator.validate_turn_and_remember(Turn::HitHookLeft));

        assert!(validator.validate_turn_and_remember(Turn::MoveForward));
        assert!(validator.validate_turn_and_remember(Turn::HitHookLeft));

        assert!(validator.validate_turn_and_remember(Turn::MoveBackward));
        assert!(validator.validate_turn_and_remember(Turn::HitHookLeft));

        assert!(validator.validate_turn_and_remember(Turn::Pass));
        assert!(validator.validate_turn_and_remember(Turn::HitHookLeft));
    }
}
