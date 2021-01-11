use crate::{Bullet, BulletDirection, GameObj, Position, StepResult};

pub struct Bunker {
    position: Position,
    stable: [[bool; Bunker::WIDTH]; Bunker::HEIGHT],
}

impl Bunker {
    const STABILITY: [[bool; Self::WIDTH]; Self::HEIGHT] = [
        [true, true, true],
        [true, true, true],
        [true, false, true],
    ];

    pub fn at_position(position: Position) -> Self {
        Self {
            position,
            stable: Self::STABILITY,
        }
    }

    fn process_bullet(&mut self, bullet: &Bullet) {
        let x = bullet.position.x - self.position.x;
        assert!(matches!(x, 0..=Self::WIDTH));
        let y = bullet.position.y - self.position.y;
        assert!(y == 0 || y == Self::HEIGHT - 1);

        let range = match bullet.direction {
            BulletDirection::Upwards => (0..Self::HEIGHT).step_by(1),
            BulletDirection::Downwards => (Self::HEIGHT + 1..=0).step_by(-1)
        };

        for y in range {
            let is_stable = &mut self.stable[x][y];
            if is_stable {
                *is_stable = false;
                return;
            }
        }
    }

    fn is_destroyed(&self) -> bool {
        self.stable
            .iter()
            .all(|row| {
                row
                    .iter()
                    .all(|b| !*b)
            })
    }
}

impl GameObj for Bunker {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;

    fn step(&mut self, hit: &mut Option<Bullet>) -> StepResult {
        let survived = match hit {
            Some(bullet) => {
                self.process_bullet(bullet);
                !self.is_destroyed()
            }
            None => true
        };

        StepResult {
            survived,
            shot: None,
        }
    }
}
