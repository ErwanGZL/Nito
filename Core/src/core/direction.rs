use crate::Vector2D;

#[derive(Clone, Copy)]
pub enum Direction {
    N(u32),
    NE(u32),
    E(u32),
    SE(u32),
    S(u32),
    SW(u32),
    W(u32),
    NW(u32),
}

impl Direction {
    pub fn factor(&self) -> Vector2D<i8> {
        match self {
            Self::N(_) => Vector2D { x: 0, y: -1 },
            Self::NE(_) => Vector2D { x: 1, y: -1 },
            Self::E(_) => Vector2D { x: 1, y: 0 },
            Self::SE(_) => Vector2D { x: 1, y: 1 },
            Self::S(_) => Vector2D { x: 0, y: 1 },
            Self::SW(_) => Vector2D { x: -1, y: 1 },
            Self::W(_) => Vector2D { x: -1, y: 0 },
            Self::NW(_) => Vector2D { x: -1, y: -1 },
        }
    }
    pub fn distance(&self) -> u32 {
        match self {
            Self::N(d) => *d,
            Self::NE(d) => *d,
            Self::E(d) => *d,
            Self::SE(d) => *d,
            Self::S(d) => *d,
            Self::SW(d) => *d,
            Self::W(d) => *d,
            Self::NW(d) => *d,
        }
    }
    pub fn set_distance(&mut self, distance: u32) {
        match self {
            Self::N(d) => *d = distance,
            Self::NE(d) => *d = distance,
            Self::E(d) => *d = distance,
            Self::SE(d) => *d = distance,
            Self::S(d) => *d = distance,
            Self::SW(d) => *d = distance,
            Self::W(d) => *d = distance,
            Self::NW(d) => *d = distance,
        }
    }
}
