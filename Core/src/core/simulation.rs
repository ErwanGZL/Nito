use std::fmt::{Display, Formatter};

use rand::seq::SliceRandom;

use crate::Cell;
use crate::Element;
use crate::Vector2D;
use crate::{Action, Direction};

pub struct Simulation {
    dimensions: Vector2D<usize>,
    pub world: Vec<Vec<Cell>>,
}

impl Display for Simulation {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in self.world.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Simulation {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            dimensions: Vector2D { x, y },
            world: vec![vec![Cell::new(Element::Air); x]; y],
        }
    }
    pub fn update(&mut self) {
        let buffer = self.world.clone();
        for (y, row) in buffer.iter().enumerate() {
            let mut shuffle = (0..row.len()).collect::<Vec<usize>>();
            shuffle.shuffle(&mut rand::thread_rng());

            for x in shuffle {
                let action = row[x].update(Vector2D { x, y }, &self);
                self.apply_actions(action);
            }
        }
    }
    pub fn dump(&mut self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        let mut body: Vec<u8> = vec![];
        data.extend((self.dimensions.x as u16).to_le_bytes());
        data.extend((self.dimensions.y as u16).to_le_bytes());
        let buffer = self.world.clone();
        for (y, row) in buffer.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell.updated() {
                    false => {}
                    _other => {
                        body.extend((x as u16).to_le_bytes());
                        body.extend((y as u16).to_le_bytes());
                        body.push(cell.element().to_byte());
                        self.world[y][x].reset_update();
                    }
                }
            }
        }
        data.extend(((body.len() / 5) as u32).to_le_bytes());
        data.extend(body);
        data
    }

    fn in_bounds(&self, position: &Vector2D<usize>) -> bool {
        position.x < self.dimensions.x && position.y < self.dimensions.y
    }

    pub fn at(
        &self,
        from: &Vector2D<usize>,
        direction: Direction,
    ) -> Option<(Element, Vector2D<usize>)> {
        let factor = direction.factor();
        let destination = Vector2D {
            x: (from.x as isize + direction.distance() as isize * factor.x as isize) as usize,
            y: (from.y as isize + direction.distance() as isize * factor.y as isize) as usize,
        };
        if !self.in_bounds(&from) || !self.in_bounds(&destination) {
            return None;
        }
        Some((
            self.world[destination.y][destination.x].element(),
            destination,
        ))
    }

    pub fn apply_actions(&mut self, action: Option<Action>) {
        match action {
            Some(Action::Swap(from, to)) => {
                let temp = self.world[from.y][from.x];
                self.world[from.y][from.x] = self.world[to.y][to.x];
                self.world[to.y][to.x] = temp;

                self.world[from.y][from.x].set_update();
                self.world[to.y][to.x].set_update();
            }
            None => {
                // println!("No action");
            }
        }
    }
}
