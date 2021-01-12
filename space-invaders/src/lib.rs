#![feature(drain_filter)]
#![feature(bool_to_option)]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use crate::alien::Aliens;
use crate::bullet::{Bullet, Shot};
use crate::bunker::Bunkers;
use crate::cannon::Cannon;

mod alien;
mod bullet;
mod bunker;
mod cannon;

pub type Unit = usize;
pub type Score = i64;
pub type Survived = bool;
pub type AbsorbedHit = bool;

#[derive(Clone, Copy, Default, Debug)]
pub struct Position {
    x: Unit,
    y: Unit,
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
        let overlaps_on_x = self.x <= other.x + other_width && other.x <= self.x + self_width;
        let overlaps_on_y = self.y <= other.y + other_height && other.y <= self.x + self_height;

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

pub trait WouldHit {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut dyn GetHit>;
}

pub trait GetHit: WouldHit {
    fn hit(&mut self, _bullet: &Bullet, _score: &mut Score) -> HitResult {
        HitResult::default()
    }
}

impl<T: WouldHit> WouldHit for &mut Option<T> {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut dyn GetHit> {
        match self {
            Some(inner) => inner.would_hit(bullet),
            None => None
        }
    }
}

impl<T: GetHit> GetHit for &mut Option<T> {
    fn hit(&mut self, bullet: &Bullet, score: &mut i64) -> HitResult {
        match self {
            Some(go) => go.hit(bullet, score),
            None => HitResult { survived: false, absorbed_bullet: false }
        }
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

pub enum Instruction {
    MoveRight,
    MoveLeft,
    Shoot,
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
    pub const HEIGHT: Unit = 40;
    pub const WIDTH: Unit = 75;
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

    pub fn cannon(&self) -> &Cannon {
        &self.cannon
    }

    pub fn score(&self) -> i64 {
        self.score
    }

    pub fn lives(&self) -> usize {
        self.lives
    }

    pub fn step(&mut self, instruction: Instruction) -> Survived {
        match instruction {
            Instruction::MoveRight => self.cannon.move_right(),
            Instruction::MoveLeft => self.cannon.move_left(),
            Instruction::Shoot => self.bullets.push(self.cannon.shoot()),
            Instruction::None => {}
        }

        // todo: handle all aliens died
        let aliens_sr = self.aliens.step();
        let mut survived = true;
        let score = &mut self.score;
        let lives = &mut self.lives;
        let cannon = &mut self.cannon;
        let aliens = &mut self.aliens;

        self.bullets.drain_filter(|bullet| {
            bullet.step();

            // bullet is out of field
            if !Self::overlaps(bullet) {
                *score -= 1;
                return true;
            }

            // bullet hit the Cannon
            if let Some(_) = cannon.would_hit(bullet) {
                *lives -= 1;
                survived = false;
                return true;
            }

            // bullet hit an Alien
            if let Some(alien) = aliens.would_hit(bullet) {
                if alien.hit(bullet, score).absorbed_bullet {
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
