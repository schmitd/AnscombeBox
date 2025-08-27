use crate::state::Point2;
use ndarray::*;

pub struct Player {
    pub position: Point2,
    pub bitmap: Array2<bool>,
}

impl Player {
    pub fn new(position: Point2, bitmap: Array2<bool>) -> Self {
        Self { position, bitmap }
    }
}
