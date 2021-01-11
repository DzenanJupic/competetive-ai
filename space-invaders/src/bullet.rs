use crate::{GameObj, Position, StepResult};
use crate::alien::AlienType;

pub struct Bullet {
    position: Position,
    direction: BulletDirection,
    alien_type: Option<AlienType>,
}

pub enum BulletDirection {
    Upwards,
    Downwards,
}

impl Bullet {
    pub(crate) fn player_at_position(position: Position) -> Self {
        Self {
            position,
            direction: BulletDirection::Upwards,
            alien_type: None,
        }
    }

    pub(crate) fn alien_at_position(position: Position, alien_type: AlienType) -> Self {
        Self {
            position,
            direction: BulletDirection::Downwards,
            alien_type: Some(alien_type),
        }
    }
}

impl GameObj for Bullet {
    const WIDTH: usize = 22;
    const HEIGHT: usize = 48;

    fn step(&mut self, hit: &mut Option<Bullet>) -> StepResult {
        assert!(hit.is_none());
        match self.direction {
            BulletDirection::Upwards => self.position.y -= 1,
            BulletDirection::Downwards => self.position.y += 1,
        }
        StepResult { survived: true, shot: None }
    }
}
