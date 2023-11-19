use std::fmt::{Display, Formatter};

use crate::Element;
use crate::{Action, Simulation, Vector2D};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    element: Element,
    updated: bool,
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
            updated: false,
        }
    }
    pub fn update(&self, position: Vector2D<usize>, simulation: &Simulation) -> Option<Action> {
        self.element.update(position, simulation)
    }

    pub fn element(&self) -> Element {
        self.element
    }

    pub fn set_update(&mut self) {
        self.updated = true;
    }

    pub fn reset_update(&mut self) {
        self.updated = false;
    }

    pub fn updated(&self) -> bool {
        self.updated
    }
}
