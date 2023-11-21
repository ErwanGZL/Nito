use std::fmt::{Display, Formatter};

use rand::Rng;

use crate::{Action, Direction, Simulation, Vector2D};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Air,
    Water,
    Sand,
    Wood,
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
            Self::Wood => 10.0,
        }
    }
    fn flammability(&self) -> f64 {
        match self {
            Self::Air => 0.0,
            Self::Water => 0.0,
            Self::Sand => 0.0,
            Self::Wood => 1.0,
        }
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
            Self::Wood => {
                write!(f, "▓").unwrap();
            }
        }
        Ok(())
    }
}

impl Element {
    pub fn from_byte(byte: u8) -> Result<Self, ()> {
        match byte {
            0 => Ok(Self::Air),
            1 => Ok(Self::Water),
            2 => Ok(Self::Sand),
            3 => Ok(Self::Wood),
            _ => Err(()),
        }
    }
    pub fn to_byte(&self) -> u8 {
        match self {
            Self::Air => 0,
            Self::Water => 1,
            Self::Sand => 2,
            Self::Wood => 3,
        }
    }

    pub fn update(&self, from: Vector2D<usize>, simulation: &Simulation) -> Option<Action> {
        let mut rng = rand::thread_rng();

        match self {
            Self::Air => {}
            Self::Water => {
                if let Some(destination) = self.try_move_to(from, Direction::S(1), simulation) {
                    return Some(Action::Swap(from, destination));
                }
                if let Some(destination) = self.try_move_sides(from, Direction::S(1), simulation) {
                    return Some(Action::Swap(from, destination));
                }
                if let Some(destination) = self.try_spread(from, 5, simulation) {
                    return Some(Action::Swap(from, destination));
                }
            }
            Self::Sand => {
                if let Some(destination) = self.try_move_to(from, Direction::S(2), simulation) {
                    return Some(Action::Swap(from, destination));
                }
                let d = rng.gen_range(1..=2);
                if let Some(destination) = self.try_move_sides(from, Direction::S(d), simulation) {
                    return Some(Action::Swap(from, destination));
                }
            }
            Self::Wood => {
            }
        }
        None
    }

    fn try_move_between_opposite(
        &self,
        from: Vector2D<usize>,
        direction: Direction,
        simulation: &Simulation,
    ) -> Option<Vector2D<usize>> {
        let mut rng = rand::thread_rng();
        let first = rng.gen_bool(0.5);
        match direction {
            Direction::W(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::E(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::W(d), simulation) {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::W(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::E(d), simulation) {
                        return Some(destination);
                    }
                }
            }
            Direction::SW(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::NE(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::SW(d), simulation)
                    {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::SW(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::NE(d), simulation)
                    {
                        return Some(destination);
                    }
                }
            }
            Direction::S(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::N(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::S(d), simulation) {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::S(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::N(d), simulation) {
                        return Some(destination);
                    }
                }
            }
            Direction::SE(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::NW(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::SE(d), simulation)
                    {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::SE(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::NW(d), simulation)
                    {
                        return Some(destination);
                    }
                }
            }
            Direction::E(d) => {
                self.try_move_between_opposite(from, Direction::W(d), simulation);
            }
            Direction::NE(d) => {
                self.try_move_between_opposite(from, Direction::SW(d), simulation);
            }
            Direction::N(d) => {
                self.try_move_between_opposite(from, Direction::S(d), simulation);
            }
            Direction::NW(d) => {
                self.try_move_between_opposite(from, Direction::SE(d), simulation);
            }
        }
        None
    }

    fn try_spread(
        &self,
        from: Vector2D<usize>,
        distance: u32,
        simulation: &Simulation,
    ) -> Option<Vector2D<usize>> {
        let mut rng = rand::thread_rng();
        let first = rng.gen_bool(0.5);
        let mut destination = None;
        for i in 1..=distance {
            if let Some(a) = self.try_move_to(
                from,
                if first {
                    Direction::W(i)
                } else {
                    Direction::E(i)
                },
                simulation,
            ) {
                if let Some((e, _b)) = simulation.at(&a, Direction::S(1)) {
                    if e.density() <= self.density() {
                        return Some(a);
                    }
                }
                destination = Some(a);
            }
        }
        destination
    }

    fn try_move_to(
        &self,
        from: Vector2D<usize>,
        mut direction: Direction,
        simulation: &Simulation,
    ) -> Option<Vector2D<usize>> {
        let distance = direction.distance();
        let mut destination = None;
        for i in 1..=distance {
            direction.set_distance(i);
            destination = match simulation.at(&from, direction) {
                Some((e, d)) => {
                    if e.density() >= self.density() {
                        return destination;
                    }
                    Some(d)
                }
                None => {
                    return destination;
                }
            };
        }
        destination
    }

    fn try_move_sides(
        &self,
        from: Vector2D<usize>,
        orientation: Direction,
        simulation: &Simulation,
    ) -> Option<Vector2D<usize>> {
        let mut rng = rand::thread_rng();
        let first = rng.gen_bool(0.5);
        match orientation {
            Direction::N(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::NW(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::NE(d), simulation)
                    {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::NE(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::NW(d), simulation)
                    {
                        return Some(destination);
                    }
                }
            }
            Direction::S(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::SW(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::SE(d), simulation)
                    {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::SE(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::SW(d), simulation)
                    {
                        return Some(destination);
                    }
                }
            }
            Direction::E(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::NE(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::SE(d), simulation)
                    {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::SE(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::NE(d), simulation)
                    {
                        return Some(destination);
                    }
                }
            }
            Direction::W(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::NW(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::SW(d), simulation)
                    {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::SW(d), simulation)
                    {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::NW(d), simulation)
                    {
                        return Some(destination);
                    }
                }
            }
            Direction::NW(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::N(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::W(d), simulation) {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::W(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::N(d), simulation) {
                        return Some(destination);
                    }
                }
            }
            Direction::NE(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::N(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::E(d), simulation) {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::E(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::N(d), simulation) {
                        return Some(destination);
                    }
                }
            }
            Direction::SW(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::S(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::W(d), simulation) {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::W(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::S(d), simulation) {
                        return Some(destination);
                    }
                }
            }
            Direction::SE(d) => {
                if first {
                    if let Some(destination) = self.try_move_to(from, Direction::S(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::E(d), simulation) {
                        return Some(destination);
                    }
                } else {
                    if let Some(destination) = self.try_move_to(from, Direction::E(d), simulation) {
                        return Some(destination);
                    }
                    if let Some(destination) = self.try_move_to(from, Direction::S(d), simulation) {
                        return Some(destination);
                    }
                }
            }
        }
        None
    }
}
