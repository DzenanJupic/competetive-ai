use crate::{Bullet, GameObj, GetHit, HitResult, PlayField, Position, Unit, WouldHit};
use crate::cannon::Cannon;

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
            y: PlayField::HEIGHT - (Cannon::HEIGHT * 3),
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
}

impl GameObj for Bunkers {
    const WIDTH: usize = Bunker::WIDTH * Self::BUNKERS + (Self::BUNKERS - 1) * Self::GRID_GAP;
    const HEIGHT: usize = Bunker::HEIGHT;

    fn position(&self) -> Position {
        self.position
    }
}

impl WouldHit for Bunkers {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut dyn GetHit> {
        self.bunkers
            .iter_mut()
            .filter_map(Option::as_mut)
            .find_map(|b| b.would_hit(bullet))
    }
}

pub struct Bunker {
    position: Position,
    stable: [[bool; Bunker::HEIGHT]; Bunker::WIDTH],
}

impl Bunker {
    const STABILITY: [[bool; Self::HEIGHT]; Self::WIDTH] = [
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

    fn is_destroyed(&self) -> bool {
        self.stable
            .iter()
            .all(|row| {
                row
                    .iter()
                    .all(|is_stable| !*is_stable)
            })
    }
}

impl GameObj for Bunker {
    const WIDTH: usize = 3;
    const HEIGHT: usize = 3;

    fn position(&self) -> Position {
        self.position
    }
}

impl WouldHit for Bunker {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut dyn GetHit> {
        if !self.overlaps(bullet) { return None; }
        let x = bullet.position().x - self.position.x;
        let y = bullet.position().y - self.position.y;

        self
            .stable[x][y]
            .then_some(self)
    }
}

impl GetHit for Bunker {
    fn hit(&mut self, bullet: &Bullet, _score: &mut i64) -> HitResult {
        let x = bullet.position().x - self.position.x;
        let y = bullet.position().y - self.position.y;
        self.stable[x][y] = false;

        HitResult {
            survived: !self.is_destroyed(),
            absorbed_bullet: true,
        }
    }
}
