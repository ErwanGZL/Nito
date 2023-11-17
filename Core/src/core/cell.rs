use std::fmt::{Display, Formatter};

use crate::{Action, Direction, Simulation, Vector2D};
use crate::Element;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    element: Element,
    clock: u8,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl Cell {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            clock: 0,
        }
    }
    pub fn update(&self, position: Vector2D<usize>, simulation: &Simulation) -> Option<Action> {
        self.element.update(position, simulation)
    }

    pub fn element(&self) -> Element {
        self.element
    }

    pub fn clock(&self) -> u8 {
        self.clock
    }

    pub fn add_clock(&mut self) {
        self.clock += 1;
    }

    pub fn reset_clock(&mut self) {
        self.clock = 0;
    }
}
