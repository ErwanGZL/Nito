extern crate core;

pub use action::Action;
#[allow(dead_code, unused)]
pub use config::open_config;
pub use direction::Direction;
pub use element::Element;
pub use cell::Cell;
pub use simulation::Simulation;

mod action;
mod config;
mod direction;
mod element;
mod cell;
mod simulation;

#[derive(Clone, Copy, Debug)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}
