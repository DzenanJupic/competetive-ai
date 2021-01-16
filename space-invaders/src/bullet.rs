use alloc::vec::Vec;

use crate::{GameObj, Position, Step, StepResult};
use crate::alien::AlienType;

pub enum Shot {
    One(Bullet),
    Many(Vec<Bullet>),
    None,
}

#[derive(Clone, Debug)]
pub struct Bullet {
    position: Position,
    direction: BulletDirection,
    // just in case, we want different bullets for different aliens
    // otherwise, this field is useless
    alien_type: Option<AlienType>,
}

#[derive(Clone, Copy, Debug)]
pub enum BulletDirection {
    Upwards,
    Downwards,
}

impl Bullet {
    pub(crate) fn is_alien_bullet(&self) -> bool {
        self.alien_type.is_some()
    }

    pub(crate) fn direction(&self) -> BulletDirection {
        self.direction
    }

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

    pub fn directional_position(&self) -> Position {
        match self.direction {
            BulletDirection::Upwards => self.position,
            BulletDirection::Downwards => {
                let mut p = self.position;
                p.y += Self::HEIGHT - 1;
                p
            }
        }
    }
}

impl GameObj for Bullet {
    const WIDTH: usize = 1;
    const HEIGHT: usize = 3;

    fn position(&self) -> Position {
        self.position
    }
}

impl Step for Bullet {
    fn step(&mut self) -> StepResult {
        let y = match self.direction {
            BulletDirection::Upwards => self.position.y.checked_sub(1),
            BulletDirection::Downwards => self.position.y.checked_add(1),
        };

        let survived = match y {
            Some(y) => {
                self.position.y = y;
                true
            },
            None => false
        };

        StepResult {
            survived,
            shot: Shot::None,
        }
    }
}
