pub mod domino;

use super::board::{Board, GridPosition};

pub trait Piece {
    fn new(board: &Board) -> Self;
    fn positions(&self) -> Vec<GridPosition>;

    fn up(&mut self, board: &Board);
    fn left_slide(&mut self, board: &Board);
    fn right_slide(&mut self, board: &Board);
    fn drop(&mut self, board: &Board);
    fn clock_rotation(&mut self, board: &Board);
    fn anticlock_rotation(&mut self, board: &Board);
    fn fix(&self, board: &Board) -> bool;
}
