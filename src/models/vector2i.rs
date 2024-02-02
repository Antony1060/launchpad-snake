use std::borrow::Borrow;
use std::cmp::{max, min};

use crate::models::direction::Direction;
use crate::{PAD_HEIGHT, PAD_WIDTH};

#[derive(Debug, Clone)]
pub struct Vector2i {
    pub x: u8,
    pub y: u8,
}

impl Vector2i {
    pub fn move_with<D: Borrow<Direction>>(&self, direction: D) -> Vector2i {
        let direction = direction.borrow();

        match direction {
            Direction::Up => Vector2i {
                x: self.x,
                y: min(self.y, PAD_HEIGHT - 2) + 1,
            },
            Direction::Down => Vector2i {
                x: self.x,
                y: max(self.y, 1) - 1,
            },
            Direction::Left => Vector2i {
                x: max(self.x, 1) - 1,
                y: self.y,
            },
            Direction::Right => Vector2i {
                x: min(self.x, PAD_WIDTH - 2) + 1,
                y: self.y,
            },
        }
    }
}
