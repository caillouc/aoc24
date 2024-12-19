use super::position::Position;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Rigth, 
    Left,
    Up,
    Down,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rigth => write!(f, "Rigth (>)"),
            Self::Left => write!(f, "Left (<)"),
            Self::Up => write!(f, "Up (^)"),
            Self::Down => write!(f, "Down (v)"),
        }
    }
}

impl Direction {
    pub fn pos(&self) -> Position {
        match self {
            Self::Rigth => Position::from(1, 0),
            Self::Left => Position::from(-1, 0),
            Self::Up => Position::from(0, -1),
            Self::Down => Position::from(0, 1),
        }
    }
    pub fn clockwise(&self) -> Self {
        match self {
            Self::Rigth => Self::Down,
            Self::Left => Self::Up,
            Self::Up => Self::Rigth,
            Self::Down => Self::Left,
        }
    }
    pub fn counter_clockwise(&self) -> Self {
        match self {
            Self::Rigth => Self::Up,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
            Self::Down => Self::Rigth,
        }
    }
}