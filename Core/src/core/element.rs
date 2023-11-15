use std::fmt::{Display, Formatter};

use crate::{Action, Direction, Simulation, Vector2D};

#[derive(Debug, Clone, Copy)]
pub enum Element {
    Air,
    Water,
    Sand,
}

trait Physics {
    fn density(&self) -> f64;
    fn flammability(&self) -> f64;
}

impl Physics for Element {
    fn density(&self) -> f64 {
        match self {
            Self::Air => 0.0,
            Self::Water => 1.0,
            Self::Sand => 2.0,
        }
    }
    fn flammability(&self) -> f64 {
        match self {
            Self::Air => 0.0,
            Self::Water => 0.0,
            Self::Sand => 0.0,
        }
    }
}

impl Element {
    pub fn from_byte(byte: u8) -> Result<Self, ()> {
        match byte {
            0 => Ok(Self::Air),
            1 => Ok(Self::Water),
            2 => Ok(Self::Sand),
            _ => Err(()),
        }
    }
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Air => 0,
            Self::Water => 1,
            Self::Sand => 2,
        }
    }
    pub fn update(&self, position: Vector2D<usize>, simulation: &Simulation) -> Option<Action> {
        match self {
            Self::Air => {}
            Self::Water => {
                if let Some((south, target)) = simulation.at(&position, Direction::S(1)) {
                    if self.density() > south.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((sw, target)) = simulation.at(&position, Direction::SW(1)) {
                    if self.density() > sw.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((se, target)) = simulation.at(&position, Direction::SE(1)) {
                    if self.density() > se.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((w, target)) = simulation.at(&position, Direction::W(1)) {
                    if self.density() > w.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((e, target)) = simulation.at(&position, Direction::E(1)) {
                    if self.density() > e.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
            }
            Self::Sand => {
                if let Some((south, target)) = simulation.at(&position, Direction::S(1)) {
                    if self.density() > south.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((sw, target)) = simulation.at(&position, Direction::SW(1)) {
                    if self.density() > sw.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((se, target)) = simulation.at(&position, Direction::SE(1)) {
                    if self.density() > se.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
            }
        }
        None
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Air => {
                write!(f, "·").unwrap();
            }
            Self::Water => {
                write!(f, "~").unwrap();
            }
            Self::Sand => {
                write!(f, "¤").unwrap();
            }
        }
        Ok(())
    }
}
