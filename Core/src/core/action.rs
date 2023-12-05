use crate::{Direction, Vector2D};

pub enum Action {
    Move(Vector2D<usize>, Direction),
}
