use core::slice::Iter;

use crate::{Bullet, GameObj, GetHit, HitResult, PlayField, Position, Unit, WouldHit};
use crate::bullet::BulletDirection;
use crate::cannon::Cannon;

#[derive(Clone, Debug)]
pub struct Bunkers {
    position: Position,
    bunkers: [Option<Bunker>; Bunkers::BUNKERS],
}

impl Bunkers {
    pub const BUNKERS: usize = 4;
    pub const GRID_GAP: Unit = Bunker::WIDTH;

    pub fn new() -> Self {
        const BASE_POSITION: Position = Position {
            x: (PlayField::WIDTH - Bunkers::WIDTH) / 2,
            y: PlayField::HEIGHT - (Cannon::HEIGHT * 5),
        };

        Self {
            position: BASE_POSITION,
            bunkers: array_init::array_init(|col| {
                Some(Bunker::at_position(Position {
                    x: BASE_POSITION.x + col * (Bunker::WIDTH + Bunkers::GRID_GAP),
                    y: BASE_POSITION.y,
                }))
            }),
        }
    }

    pub fn iter(&self) -> Iter<'_, Option<Bunker>> {
        self.bunkers.iter()
    }
}

impl GameObj for Bunkers {
    const WIDTH: usize = Bunker::WIDTH * Self::BUNKERS + (Self::BUNKERS - 1) * Self::GRID_GAP;
    const HEIGHT: usize = Bunker::HEIGHT;

    fn position(&self) -> Position {
        self.position
    }
}

impl WouldHit<Option<Bunker>> for Bunkers {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut Option<Bunker>> {
        self.bunkers
            .iter_mut()
            .find_map(|bunker| {
                bunker
                    .as_mut()
                    .and_then(|b| b.would_hit(bullet))
                    .is_some()
                    .then_some(bunker)
            })
    }
}

#[derive(Clone, Debug)]
pub struct Bunker {
    position: Position,
    stable: [[u8; 3]; 3],
}

impl Bunker {
    const STABILITY: [[u8; 3]; 3] = [
        [2, 2, 2],
        [2, 0, 2],
        [2, 0, 2],
    ];

    pub fn at_position(position: Position) -> Self {
        Self {
            position,
            stable: Self::STABILITY,
        }
    }

    fn is_destroyed(&self) -> bool {
        self.stable
            .iter()
            .all(|row| {
                row
                    .iter()
                    .all(|&stability| stability == 0)
            })
    }
}

impl GameObj for Bunker {
    const WIDTH: usize = 24;
    const HEIGHT: usize = 18;

    fn position(&self) -> Position {
        self.position
    }
}

impl WouldHit<Bunker> for Bunker {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut Bunker> {
        if !self.overlaps(bullet) || bullet.position().y < self.position.y { return None; }

        let x = (bullet.directional_position().x - self.position.x) / (Self::WIDTH / 3);
        let y = (bullet.directional_position().y - self.position.y) / (Self::HEIGHT / 3);
        if x == 3 || y == 3 { return None; }

        (self.stable[y][x] > 0)
            .then_some(self)
    }
}

impl GetHit for Bunker {
    fn hit(&mut self, bullet: &Bullet, _score: &mut i64) -> HitResult {
        let x = (bullet.directional_position().x - self.position.x) / (Self::WIDTH / 3);
        let y = (bullet.directional_position().y - self.position.y) / (Self::HEIGHT / 3);
        self.stable[y][x] -= 1;

        HitResult {
            survived: !self.is_destroyed(),
            absorbed_bullet: true,
        }
    }
}
