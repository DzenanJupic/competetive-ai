use crate::{Bullet, GameObj, PlayField, Position, StepResult};

pub struct Cannon {
    position: Position
}

impl Cannon {
    pub fn position(&self) -> Position {
        self.position
    }

    pub(crate) fn move_right(&mut self) {
        if self.position.x + Self::WIDTH < PlayField::WIDTH - 1 {
            self.position.x += 1;
        }
    }

    pub(crate) fn move_left(&mut self) {
        if self.position.x > 0 {
            self.position.x -= 1;
        }
    }
}

impl GameObj for Cannon {
    const WIDTH: usize = 2;
    const HEIGHT: usize = 2;

    fn step(&mut self, hit: &mut Option<Bullet>) -> StepResult {
        StepResult {
            survived: hit.is_none(),
            shot: None,
        }
    }
}
