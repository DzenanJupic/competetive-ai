use alloc::vec::Vec;
use core::slice::Iter;

use rand::Rng;

use crate::{Bullet, GameObj, GetHit, HitResult, PlayField, Position, Step, StepResult, Unit, WouldHit};
use crate::bullet::Shot;

#[derive(Clone, Debug)]
pub struct Aliens {
    position: Position,
    aliens: [[Option<Alien>; Aliens::ROWS]; Aliens::COLUMNS],
}

impl Aliens {
    pub const ROWS: usize = 5;
    pub const COLUMNS: usize = 11;
    pub const GRID_GAP: Unit = Alien::WIDTH / 3;

    pub fn new() -> Self {
        const BASE_POSITION: Position = Position {
            x: (PlayField::WIDTH - Aliens::WIDTH) / 2,
            y: 0,
        };

        Self {
            position: BASE_POSITION,
            aliens: array_init::array_init(|col| {
                array_init::array_init(|row| {
                    Some(Alien {
                        alien_type: AlienType::from_row(row),
                        position: Position {
                            x: BASE_POSITION.x + col * (Alien::WIDTH + Aliens::GRID_GAP),
                            y: BASE_POSITION.y + row * (Alien::HEIGHT + Aliens::GRID_GAP),
                        },
                    })
                })
            }),
        }
    }

    pub fn iter(&self) -> Iter<'_, [Option<Alien>; 5]> {
        self.aliens.iter()
    }
}

impl GameObj for Aliens {
    const WIDTH: usize = Alien::WIDTH * Self::COLUMNS + (Self::COLUMNS - 1) * Self::GRID_GAP;
    const HEIGHT: usize = Alien::HEIGHT * Self::ROWS + (Self::ROWS - 1) * Self::GRID_GAP;

    fn position(&self) -> Position {
        self.position
    }
}

impl Step for Aliens {
    fn step(&mut self) -> StepResult {
        // todo: move aliens
        let mut one_survived = false;
        let mut shots = Vec::new();

        self.aliens
            .iter_mut()
            .map(|row| row.iter_mut())
            .flatten()
            .filter(|opt| opt.is_some())
            .map(|o| (o.as_mut().unwrap().step(), o))
            .for_each(|(sr, o)| {
                if sr.survived {
                    one_survived = true;
                    match sr.shot {
                        Shot::One(bullet) => shots.push(bullet),
                        Shot::Many(bullets) => shots.extend(bullets),
                        Shot::None => {}
                    }
                } else {
                    *o = None;
                }
            });

        StepResult {
            survived: one_survived,
            shot: Shot::Many(shots),
        }
    }
}

impl WouldHit<Option<Alien>> for Aliens {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut Option<Alien>> {
        self.aliens
            .iter_mut()
            .map(|row| row.iter_mut())
            .flatten()
            .find_map(|alien| {
                alien
                    .as_mut()
                    .and_then(|alien| alien.would_hit(bullet))
                    .is_some()
                    .then_some(alien)
            })
    }
}

#[derive(Clone, Debug)]
pub struct Alien {
    alien_type: AlienType,
    position: Position,
}

impl Alien {
    pub fn alien_type(&self) -> AlienType {
        self.alien_type
    }
}

impl GameObj for Alien {
    const WIDTH: usize = 12;
    const HEIGHT: usize = 8;

    fn position(&self) -> Position {
        self.position
    }
}

impl Step for Alien {
    fn step(&mut self) -> StepResult {
        if let AlienType::Mystery = self.alien_type {
            self.position.x += 1;
            return StepResult {
                survived: PlayField::overlaps(self),
                shot: Shot::None,
            };
        }

        // todo: only shoot, if the Alien is lowest in it's column
        let shot = rand::thread_rng()
            .gen_bool(self.alien_type.shoot_probability())
            .then(|| Shot::One(Bullet::alien_at_position(
                Position {
                    x: self.position.x + Self::WIDTH / 2,
                    y: self.position.y + Self::HEIGHT,
                },
                self.alien_type,
            )))
            .unwrap_or(Shot::None);

        StepResult {
            survived: true,
            shot,
        }
    }
}


impl WouldHit<Alien> for Alien {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut Alien> {
        self
            .overlaps(bullet)
            .then_some(self)
    }
}

impl GetHit for Alien {
    fn hit(&mut self, bullet: &Bullet, score: &mut i64) -> HitResult {
        if bullet.is_alien_bullet() {
            HitResult { survived: true, absorbed_bullet: false }
        } else {
            *score += self.alien_type.points();
            HitResult { survived: false, absorbed_bullet: true }
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
    pub fn from_row(row: usize) -> Self {
        match row {
            0 => Self::Hard,
            1..=2 => Self::Medium,
            3..=4 => Self::Easy,
            _ => unimplemented!("there may only be 5 rows of aliens")
        }
    }

    pub fn points(&self) -> i64 {
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
            Self::Hard => 0.001,
            Self::Medium => 0.0008,
            Self::Easy => 0.0005
        }
    }
}
