use std::fmt::{Display, Formatter};

struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone)]
pub enum Element {
    Air = 0,
    Water = 1,
    Sand = 2,
}

pub struct Simulation {
    dimensions: Vector2D<usize>,
    pub world: Vec<Vec<Element>>,
}

impl Display for Simulation {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.world {
            for &cell in row {
                match cell {
                    Element::Air => {
                        write!(f, "·").unwrap();
                    }
                    Element::Water => {
                        write!(f, "~").unwrap();
                    }
                    Element::Sand => {
                        write!(f, "¤").unwrap();
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Simulation {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            dimensions: Vector2D { x, y },
            world: vec![vec![Element::Air; x]; y],
        }
    }
    pub fn update(&mut self) {
        let buffer = vec![vec![Element::Air; self.dimensions.x]; self.dimensions.y];
        for (y, row) in self.world.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {}
        }
        self.world = buffer;
        todo!();
    }
    pub fn dump(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![];
        let mut body: Vec<u8> = vec![];
        data.extend((self.dimensions.x as u16).to_le_bytes());
        data.extend((self.dimensions.y as u16).to_le_bytes());
        for (y, row) in self.world.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Element::Air => {}
                    _other => {
                        body.extend((x as u16).to_le_bytes());
                        body.extend((y as u16).to_le_bytes());
                        body.push(*_other as u8);
                    }
                }
            }
        }
        data.extend(((body.len() / 5) as u32).to_le_bytes());
        println!("Header: {:?}", data);
        data.extend(body);
        data
    }
}
