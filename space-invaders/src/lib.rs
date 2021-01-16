#![feature(drain_filter, bool_to_option)]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use crate::alien::Aliens;
use crate::bullet::{Bullet, Shot};
use crate::bunker::Bunkers;
use crate::cannon::Cannon;

pub mod alien;
pub mod bullet;
pub mod bunker;
pub mod cannon;

pub type Unit = usize;
pub type Score = i64;
pub type Survived = bool;
pub type AbsorbedHit = bool;

#[derive(Clone, Copy, Default, Debug)]
pub struct Position {
    pub x: Unit,
    pub y: Unit,
}

impl Position {
    pub const fn overlaps(
        &self,
        self_width: Unit,
        self_height: Unit,
        other: Position,
        other_width: Unit,
        other_height: Unit,
    ) -> bool {
        let overlaps_on_x = self.x <= other.x + other_width - 1 && self.x + self_width - 1 >= other.x;
        let overlaps_on_y = self.y <= other.y + other_height - 1 && self.y + self_height - 1 >= other.y;

        overlaps_on_x && overlaps_on_y
    }
}

pub trait GameObj {
    const WIDTH: Unit;
    const HEIGHT: Unit;

    fn position(&self) -> Position;

    fn overlaps<O: GameObj>(&self, other: &O) -> bool {
        self
            .position()
            .overlaps(
                Self::WIDTH,
                Self::HEIGHT,
                other.position(),
                O::WIDTH,
                O::HEIGHT,
            )
    }
}

pub trait Step {
    fn step(&mut self) -> StepResult;
}

pub trait WouldHit<T>
    where T: GetHit {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut T>;
}

pub trait GetHit {
    fn hit(&mut self, _bullet: &Bullet, _score: &mut Score) -> HitResult {
        HitResult::default()
    }
}

impl<T: GetHit> GetHit for Option<T> {
    fn hit(&mut self, bullet: &Bullet, score: &mut i64) -> HitResult {
        let hr = match self {
            Some(inner) => inner.hit(bullet, score),
            None => HitResult { survived: true, absorbed_bullet: false }
        };

        if !hr.survived {
            *self = None;
        }

        hr
    }
}

pub struct StepResult {
    survived: bool,
    shot: Shot,
}

impl Default for StepResult {
    fn default() -> Self {
        Self {
            survived: true,
            shot: Shot::None,
        }
    }
}

pub struct HitResult {
    survived: bool,
    absorbed_bullet: bool,
}

impl Default for HitResult {
    fn default() -> Self {
        Self {
            survived: false,
            absorbed_bullet: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    MoveRight,
    MoveLeft,
    None,
}

pub struct PlayField {
    aliens: Aliens,
    bunkers: Bunkers,
    bullets: Vec<Bullet>,
    cannon: Cannon,

    score: Score,
    lives: usize,
    speed: Unit,
}

impl PlayField {
    pub const HEIGHT: Unit = 256;
    pub const WIDTH: Unit = 224;
    pub const PLAYER_LIVES: usize = 3;

    pub fn new() -> Self {
        Self {
            aliens: Aliens::new(),
            bunkers: Bunkers::new(),
            bullets: Vec::new(),
            cannon: Cannon::new(),
            score: 0,
            lives: Self::PLAYER_LIVES,
            speed: 1,
        }
    }

    pub fn aliens(&self) -> &Aliens {
        &self.aliens
    }

    pub fn bunkers(&self) -> &Bunkers {
        &self.bunkers
    }

    pub fn bullets(&self) -> &Vec<Bullet> { &self.bullets }

    pub fn cannon(&self) -> &Cannon {
        &self.cannon
    }

    pub fn score(&self) -> i64 {
        self.score
    }

    pub fn lives(&self) -> usize {
        self.lives
    }

    pub fn step(&mut self, instruction: Instruction, shoot: bool) -> Survived {
        match instruction {
            Instruction::MoveRight => self.cannon.move_right(),
            Instruction::MoveLeft => self.cannon.move_left(),
            Instruction::None => {}
        }

        if shoot {
            self.bullets.push(self.cannon.shoot());
        }

        // todo: handle all aliens died
        let aliens_sr = self.aliens.step();
        let mut survived = true;
        let score = &mut self.score;
        let lives = &mut self.lives;
        let cannon = &mut self.cannon;
        let aliens = &mut self.aliens;
        let bunkers = &mut self.bunkers;

        self.bullets.drain_filter(|bullet| {
            // bullet is out of field
            if !bullet.step().survived || !Self::overlaps(bullet) {
                log::info!("bullet: {:?}", bullet);
                *score -= 1;
                return true;
            }

            // bullet hit the Cannon
            if let Some(_) = cannon.would_hit(bullet) {
                log::info!("hit cannon | {}", lives);
                *lives = lives.saturating_sub(1);
                survived = false;
                return true;
            } else if bullet.position().x > cannon.position().x && bullet.position().y > cannon.position().y {
                log::info!("bullet: {:?} | cannon: {:?}", bullet, cannon)
            }

            // bullet hit an Alien
            if let Some(alien) = aliens.would_hit(bullet) {
                if alien.hit(bullet, score).absorbed_bullet {
                    return true;
                }
            }

            // bullet hit Bunker
            if let Some(bunker) = bunkers.would_hit(bullet) {
                if bunker.hit(bullet, score).absorbed_bullet {
                    return true;
                }
            }

            false
        });

        match aliens_sr.shot {
            Shot::One(bullet) => self.bullets.push(bullet),
            Shot::Many(bullets) => self.bullets.extend(bullets),
            Shot::None => {}
        }

        survived
    }

    pub fn overlaps<O: GameObj>(other: &O) -> bool {
        Position { x: 0, y: 0 }
            .overlaps(
                Self::WIDTH,
                Self::HEIGHT,
                other.position(),
                O::WIDTH,
                O::HEIGHT,
            )
    }
}
