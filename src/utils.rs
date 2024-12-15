use std::{ops::{Add, Sub}, usize};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position{x: i32, y: i32}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position{x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl Position {
    pub fn from(x: i32, y: i32) -> Self {
        Self{x, y}
    }
    pub fn from_usize(x: usize, y: usize) -> Self {
        Self{x: x as i32, y: y as i32}
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
    pub fn left(&self) -> Option<Self> {
        if self.x - 1 < 0 {return None;}
        Some(Position{x: self.x - 1, y: self.y})
    }
    pub fn rigth(&self, max_x: usize) -> Option<Self> {
        if self.x + 1 >= max_x as i32 {return None;}
        Some(Position{x: self.x + 1, y: self.y})
    }
    pub fn top(&self) -> Option<Self> {
        if self.y - 1 < 0 {return None;}
        Some(Position{x: self.x, y: self.y - 1})
    }
    pub fn bottom(&self, max_y: usize) -> Option<Self> {
        if self.y + 1 >= max_y as i32 {return None;}
        Some(Position{x: self.x, y: self.y + 1})
    }
    pub fn top_left(&self) -> Option<Self> {
        if self.x - 1 < 0 {return None;}
        if self.y - 1 < 0 {return None;}
        Some(Position{x: self.x - 1, y: self.y - 1})
    }
    pub fn bottom_left(&self, max_y: usize) -> Option<Self> {
        if self.x - 1 < 0 {return None;}
        if self.y + 1 >= max_y as i32 {return None;}
        Some(Position{x: self.x - 1, y: self.y + 1})
    }
    pub fn top_rigth(&self, max_x: usize) -> Option<Self> {
        if self.x + 1 >= max_x as i32 {return None;}
        if self.y - 1 < 0 {return None;}
        Some(Position{x: self.x + 1, y: self.y - 1})
    }
    pub fn bottom_rigth(&self, max_x: usize, max_y: usize) -> Option<Self> {
        if self.x + 1 >= max_x as i32 {return None;}
        if self.y + 1 >= max_y as i32 {return None;}
        Some(Position{x: self.x + 1, y: self.y + 1})
    }
}