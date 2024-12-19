use super::direction::Direction;
use std::{
    fmt,
    ops::{Add, Sub},
    usize,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    x: i32,
    y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Position {
    pub fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
    pub fn is_valid(&self, x_max: usize, y_max: usize) -> bool {
        self.x >= 0 && self.x < x_max as i32 && self.y >= 0 && self.y < y_max as i32
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn x_usize(&self) -> usize {
        if self.x < 0 {
            return 0;
        }
        self.x as usize
    }
    pub fn y_usize(&self) -> usize {
        if self.y < 0 {
            return 0;
        }
        self.y as usize
    }

    pub fn to(&self, dir: Direction, max_x: usize, max_y: usize) -> Option<Self> {
        let new_pos = *self + dir.pos();
        if new_pos.x >= 0 && new_pos.x < max_x as i32 && new_pos.y >= 0 && new_pos.y < max_y as i32
        {
            return Some(new_pos);
        }
        None
    }

    pub fn toto(&self, dir1: Direction, dir2: Direction, max_x: usize, max_y: usize) -> Option<Self> {
        let new_pos = *self + dir1.pos() + dir2.pos();
        if new_pos.x >= 0 && new_pos.x < max_x as i32 && new_pos.y >= 0 && new_pos.y < max_y as i32
        {
            return Some(new_pos);
        }
        None
    }
    pub fn left(&self) -> Option<Self> {
        self.to(Direction::Left, usize::MAX, usize::MAX)
    }
    pub fn rigth(&self, max_x: usize) -> Option<Self> {
        self.to(Direction::Rigth, max_x, usize::MAX)
    }
    pub fn up(&self) -> Option<Self> {
        self.to(Direction::Up, usize::MAX, usize::MAX)
    }
    pub fn down(&self, max_y: usize) -> Option<Self> {
        self.to(Direction::Down, usize::MAX, max_y)
    }
    pub fn up_left(&self) -> Option<Self> {
        self.toto(Direction::Up, Direction::Left, usize::MAX, usize::MAX)
    }
    pub fn down_left(&self, max_y: usize) -> Option<Self> {
        self.toto(Direction::Down, Direction::Left, usize::MAX, max_y)
    }
    pub fn up_rigth(&self, max_x: usize) -> Option<Self> {
        self.toto(Direction::Up, Direction::Rigth, max_x, usize::MAX)
    }
    pub fn down_rigth(&self, max_x: usize, max_y: usize) -> Option<Self> {
        self.toto(Direction::Down, Direction::Rigth, max_x, max_y)
    }
}
