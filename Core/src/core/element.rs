use std::fmt::{Display, Formatter};
use rand::Rng;

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

        let mut rng = rand::thread_rng();

        let n: bool = rng.gen_bool(0.5);

        match self {
            Self::Air => {}
            Self::Water => {
                if let Some((south, target)) = simulation.at(&position, Direction::S(1)) {
                    if self.density() > south.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((diagonal, target)) = simulation.at(&position, if n {Direction::SW(1)} else {Direction::SE(1)}) {
                    if self.density() > diagonal.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((diagonal, target)) = simulation.at(&position, if n {Direction::SE(1)} else {Direction::SW(1)}) {
                    if self.density() > diagonal.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((lateral, target)) = simulation.at(&position, if n {Direction::W(1)} else {Direction::E(1)}) {
                    if self.density() > lateral.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((lateral, target)) = simulation.at(&position, if n {Direction::E(1)} else {Direction::W(1)}) {
                    if self.density() > lateral.density() {
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
                if let Some((diagonal, target)) = simulation.at(&position, if n {Direction::SW(1)} else {Direction::SE(1)}) {
                    if self.density() > diagonal.density() {
                        return Some(Action::Swap(position, target));
                    }
                }
                if let Some((diagonal, target)) = simulation.at(&position, if n {Direction::SE(1)} else {Direction::SW(1)}) {
                    if self.density() > diagonal.density() {
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
