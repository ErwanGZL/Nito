use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::direction::Cardinal;
use crate::element::Physics;
use crate::{Action, Simulation, Vector2D};
use crate::{Direction, Element};

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub element: Element,
    pub velocity: Vector2D<i8>,
    pub lifetime: Option<u32>,
    variant: u8,
    updated: bool,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.element)
    }
}

impl Cell {
    pub fn new(element: Element) -> Self {
        let mut rng = rand::thread_rng();
        let variant = rng.gen_range(0..=255);
        let velocity = Vector2D { x: 0, y: 0 };
        Self {
            element,
            velocity,
            lifetime: None,
            variant,
            updated: false,
        }
    }
    pub fn update(&self, origin: Vector2D<usize>, sim: &Simulation) -> Option<Action> {
        let mut rng = rand::thread_rng();
        let first = rng.gen_bool(0.5);
        match self.element {
            Element::Air => {}
            Element::Water => {
                if let Some(action) = self.move_to(origin, Direction::new(Cardinal::S, 3), sim) {
                    return Some(action);
                }
                if first {
                    if let Some(action) = self.move_to(origin, Direction::new(Cardinal::SE, 3), sim)
                    {
                        return Some(action);
                    }
                    if let Some(action) = self.move_to(origin, Direction::new(Cardinal::E, 4), sim)
                    {
                        return Some(action);
                    }
                } else {
                    if let Some(action) = self.move_to(origin, Direction::new(Cardinal::SW, 3), sim)
                    {
                        return Some(action);
                    }
                    if let Some(action) = self.move_to(origin, Direction::new(Cardinal::W, 4), sim)
                    {
                        return Some(action);
                    }
                }
            }
            Element::Sand => {
                if let Some(action) = self.move_to(origin, Direction::new(Cardinal::S, 2), sim) {
                    return Some(action);
                }
                if let Some(action) = self.move_to(origin, Direction::new(Cardinal::SW, 2), sim) {
                    return Some(action);
                }
                if let Some(action) = self.move_to(origin, Direction::new(Cardinal::SE, 2), sim) {
                    return Some(action);
                }
            }
            Element::Wood => {}
        }
        None
    }

    fn move_to(
        &self,
        from: Vector2D<usize>,
        mut to: Direction,
        simulation: &Simulation,
    ) -> Option<Action> {
        let distance = to.distance;
        let mut destination: Option<Action> = None;
        for i in 1..=distance {
            to.set_distance(i);
            if let Some((cell, _)) = simulation.at(&from, to) {
                if self.element.density() > cell.element.density() {
                    destination = Some(Action::Move(from, to));
                    continue;
                }
            }
            break;
        }
        destination
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
