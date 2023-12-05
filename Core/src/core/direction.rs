use crate::Vector2D;

#[derive(Clone, Copy, Debug)]
pub struct Direction {
    pub cardinal: Cardinal,
    pub distance: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum Cardinal {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction {
    pub fn factor(&self) -> Vector2D<i8> {
        match self.cardinal {
            Cardinal::N => Vector2D { x: 0, y: -1 },
            Cardinal::NE => Vector2D { x: 1, y: -1 },
            Cardinal::E => Vector2D { x: 1, y: 0 },
            Cardinal::SE => Vector2D { x: 1, y: 1 },
            Cardinal::S => Vector2D { x: 0, y: 1 },
            Cardinal::SW => Vector2D { x: -1, y: 1 },
            Cardinal::W => Vector2D { x: -1, y: 0 },
            Cardinal::NW => Vector2D { x: -1, y: -1 },
        }
    }

    pub fn new(cardinal: Cardinal, distance: u32) -> Self {
        Self { cardinal, distance }
    }

    pub fn distance(&self) -> u32 {
        self.distance
    }

    pub fn set_distance(&mut self, distance: u32) {
        self.distance = distance;
    }
}
