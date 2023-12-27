use std::ops::{Add, Sub};

use strum_macros::EnumIter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumIter)]
pub enum Direction4 {
    East,
    South,
    West,
    North,
}

impl Direction4 {
    pub fn rotate_ccw(self) -> Self {
        match self {
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::North => Self::West,
        }
    }
    pub fn reflect_h(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::South => Self::South,
            Self::West => Self::East,
            Self::North => Self::North,
        }
    }
    pub fn opposite(self) -> Self {
        self.rotate_ccw().rotate_ccw()
    }
    pub fn rotate_cw(self) -> Self {
        self.opposite().rotate_ccw()
    }
    pub fn reflect_v(self) -> Self {
        self.reflect_h().opposite()
    }
    pub fn reflect_nesw(self) -> Self {
        self.rotate_ccw().reflect_h()
    }
    pub fn reflect_nwse(self) -> Self {
        self.reflect_h().rotate_ccw()
    }

    pub fn advance_by<T: Add<Output = T> + Sub<Output = T>>(self, (x, y): (T, T), by: T) -> (T, T) {
        match self {
            Self::East => (x + by, y),
            Self::South => (x, y + by),
            Self::West => (x - by, y),
            Self::North => (x, y - by),
        }
    }

    pub fn advance<T: Add<Output = T> + Sub<Output = T> + From<bool>>(self, pos: (T, T)) -> (T, T) {
        self.advance_by(pos, T::from(true))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Direction8 {
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    North,
    NorthEast,
}

impl Direction8 {
    pub fn rotate_ccw(self) -> Self {
        match self {
            Self::East => Self::NorthEast,
            Self::SouthEast => Self::East,
            Self::South => Self::SouthEast,
            Self::SouthWest => Self::South,
            Self::West => Self::SouthWest,
            Self::NorthWest => Self::West,
            Self::North => Self::NorthWest,
            Self::NorthEast => Self::North,
        }
    }
    pub fn reflect_h(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::SouthEast => Self::SouthWest,
            Self::South => Self::South,
            Self::SouthWest => Self::SouthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::NorthEast,
            Self::North => Self::North,
            Self::NorthEast => Self::NorthWest,
        }
    }
    pub fn opposite(self) -> Self {
        self.rotate_ccw().rotate_ccw().rotate_ccw().rotate_ccw()
    }
    pub fn rotate_cw(self) -> Self {
        self.opposite().rotate_ccw().rotate_ccw().rotate_ccw()
    }
    pub fn reflect_v(self) -> Self {
        self.reflect_h().opposite()
    }
    pub fn reflect_nesw(self) -> Self {
        self.rotate_ccw().rotate_ccw().reflect_h()
    }
    pub fn reflect_nwse(self) -> Self {
        self.reflect_h().rotate_ccw().rotate_ccw()
    }
    pub fn advance<T: Add<Output = T> + Sub<Output = T> + From<bool>>(
        self,
        (x, y): (T, T),
    ) -> (T, T) {
        match self {
            Self::East => (x + T::from(true), y),
            Self::SouthEast => (x + T::from(true), y + T::from(true)),
            Self::South => (x, y + T::from(true)),
            Self::SouthWest => (x - T::from(true), y + T::from(true)),
            Self::West => (x - T::from(true), y),
            Self::NorthWest => (x - T::from(true), y - T::from(true)),
            Self::North => (x, y - T::from(true)),
            Self::NorthEast => (x + T::from(true), y - T::from(true)),
        }
    }
}
