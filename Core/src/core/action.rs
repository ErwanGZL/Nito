use crate::{Direction, Element, Vector2D};

pub enum Action {
    Idle(),
    Move(Vector2D<usize>, Direction),
    Burn(Vector2D<usize>),
    Eat(Vector2D<usize>, Element),
    Die(Vector2D<usize>, Element),
    Grow(Vector2D<usize>),
}
