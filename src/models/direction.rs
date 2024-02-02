pub const UP_KEY: (u8, u8) = (1, 6);
pub const DOWN_KEY: (u8, u8) = (1, 7);
pub const LEFT_KEY: (u8, u8) = (0, 7);
pub const RIGHT_KEY: (u8, u8) = (2, 7);
pub const RESET_KEY: (u8, u8) = (7, 7);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
