use crate::{Bullet, GameObj, GetHit, PlayField, Position, WouldHit};

#[derive(Clone, Debug)]
pub struct Cannon {
    position: Position
}

impl Cannon {
    pub const fn new() -> Self {
        const BASE_POSITION: Position = Position {
            x: (PlayField::WIDTH - Cannon::WIDTH) / 2,
            y: PlayField::HEIGHT - Cannon::HEIGHT,
        };

        Self {
            position: BASE_POSITION
        }
    }

    pub(crate) fn move_right(&mut self) {
        if self.position.x + Self::WIDTH < PlayField::WIDTH - 1 {
            self.position.x += 1;
        }
    }

    pub(crate) fn move_left(&mut self) {
        if self.position.x > 0 {
            self.position.x -= 1;
        }
    }

    pub(crate) fn shoot(&self) -> Bullet {
        Bullet::player_at_position(Position {
            x: self.position.x + Self::WIDTH / 2,
            y: self.position.y + 1,
        })
    }
}

impl GameObj for Cannon {
    const WIDTH: usize = 15;
    const HEIGHT: usize = 8;

    fn position(&self) -> Position {
        self.position
    }
}

impl WouldHit<Cannon> for Cannon {
    fn would_hit(&mut self, bullet: &Bullet) -> Option<&mut Cannon> {
        (bullet.is_alien_bullet() && self.overlaps(bullet))
            .then_some(self)
    }
}

impl GetHit for Cannon {}
