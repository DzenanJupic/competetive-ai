#![feature(drain_filter, step_by)]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use core::ops::Range;

use rand::Rng;

use crate::alien::{Alien, AlienType};
use crate::bullet::Bullet;
use crate::bunker::Bunker;
use crate::cannon::Cannon;

mod alien;
mod bullet;
mod bunker;
mod cannon;

pub type Unit = usize;
pub type Survived = bool;
pub type AbsorbedHit = bool;

#[derive(Clone, Copy, Default, Debug)]
pub struct Position {
    x: Unit,
    y: Unit,
}

pub trait GameObj {
    const WIDTH: Unit;
    const HEIGHT: Unit;

    fn step(&mut self, hit: &mut Option<Bullet>) -> StepResult;
}

pub struct StepResult {
    survived: bool,
    shot: Option<Bullet>,
}

pub enum Instruction {
    MoveRight,
    MoveLeft,
    Shoot,
    None,
}

pub struct PlayField {
    aliens: [[Option<Alien>; PlayField::ALIEN_COLUMNS]; PlayField::ALIEN_ROWS],
    bunkers: [Option<Bunker>; PlayField::BUNKERS],
    bullets: Vec<Bullet>,
    cannon: Cannon,

    score: u64,
    lives: usize,
    speed: Unit,
}

impl PlayField {
    pub const HEIGHT: Unit = 22;
    pub const WIDTH: Unit = 48;
    pub const BUNKERS: usize = 4;
    pub const ALIEN_ROWS: usize = 5;
    pub const ALIEN_COLUMNS: usize = 11;
    pub const PLAYER_LIVES: usize = 3;

    pub fn new() -> Self {
        Self {
            aliens: Aliens::new(),
            bunkers: Self::default_bunkers(),
            bullets: Vec::new(),
            cannon: Cannon::default(),
            score: 0,
            lives: Self::PLAYER_LIVES,
            speed: 1,
        }
    }

    pub fn aliens(&self) -> &Aliens {
        &self.aliens
    }

    pub fn bunkers(&self) -> &[Option<Bunker>; Self::BUNKERS] {
        &self.bunkers
    }

    pub fn cannon(&self) -> &Cannon {
        &self.cannon
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn lives(&self) -> usize {
        self.lives
    }

    pub fn step(&mut self, instruction: Instruction) -> Survived {
        match instruction {
            Instruction::MoveRight => self.cannon.move_right(),
            Instruction::MoveLeft => self.cannon.move_left(),
            Instruction::Shoot => {
                self.bullets.push(Bullet::player_at_position(self.cannon.position()))
            }
            Instruction::None => {}
        }

        for bullet in self.bullets.iter_mut() {
            bullet.step(&mut None);
        }


        todo!()
    }
}
