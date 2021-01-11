use rand::Rng;

use crate::{Bullet, GameObj, Instruction, Position, StepResult, Survived};

pub struct Alien {
    alien_type: AlienType,
    position: Position,
}

impl GameObj for Alien {
    const WIDTH: usize = 4;
    const HEIGHT: usize = 4;

    fn step(&mut self, hit: Option<Position>) -> StepResult {
        let shot = match hit {
            None if rand::thread_rng().gen_bool(self.alien_type.shoot_probability()) => {
                Some(Bullet::alien_at_position(self.position, self.alien_type))
            }
            _ => None
        };

        StepResult {
            survived: hit.is_none(),
            shot,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AlienType {
    Mystery,
    Hard,
    Medium,
    Easy,
}

impl AlienType {
    pub fn from_index(row: usize, _col: usize) -> Self {
        match row {
            0 => Self::Hard,
            1..=2 => Self::Medium,
            3..=4 => Self::Easy,
            _ => unimplemented!("there may only be 5 rows of aliens")
        }
    }

    pub fn points(&self) -> u8 {
        match self {
            Self::Mystery => rand::thread_rng().gen_range(10..100),
            Self::Hard => 30,
            Self::Medium => 20,
            Self::Easy => 10
        }
    }

    pub fn shoot_probability(&self) -> f64 {
        match self {
            Self::Mystery => 0.,
            Self::Hard => 0.5,
            Self::Medium => 0.3,
            Self::Easy => 0.2
        }
    }
}
