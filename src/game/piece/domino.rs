use super::super::board::{Board, GridPosition};
use super::Piece;

pub struct Domino {
    pub cell1: GridPosition,
    pub cell2: GridPosition,
}

// Implement the `Piece` trait for `Domino`
impl Piece for Domino {
    fn new(board: &Board) -> Self {
        Domino {
            cell1: GridPosition::new(board.width as i32 / 2 - 1, board.height as i32 - 1),
            cell2: GridPosition::new(board.width as i32 / 2, board.height as i32 - 1),
        }
    }

    fn positions(&self) -> Vec<GridPosition> {
        vec![self.cell1, self.cell2]
    }

    fn left_slide(&mut self, board: &Board) {
        // left side
        if self.cell1.x == 0 || self.cell2.x == 0 {
            return;
        }

        // filled cells
        if !board.left(self.cell1).empty() || !board.left(self.cell2).empty() {
            return;
        }

        self.cell1.x -= 1;
        self.cell2.x -= 1;
    }

    fn right_slide(&mut self, board: &Board) {
        // right side
        if self.cell1.x == board.width - 1 || self.cell2.x == board.width - 1 {
            return;
        }

        // filled cells
        if !board.right(self.cell1).empty() || !board.right(self.cell2).empty() {
            return;
        }

        self.cell1.x += 1;
        self.cell2.x += 1;
    }

    fn up(&mut self, board: &Board) {
        if self.cell1.y == board.height - 1 || self.cell2.y == board.height - 1 {
            return;
        }

        // filled cells
        if !board.up(self.cell1).empty() || !board.up(self.cell2).empty() {
            return;
        }

        self.cell1.y += 1;
        self.cell2.y += 1;
    }

    fn drop(&mut self, board: &Board) {
        if self.cell1.y == 0 || self.cell2.y == 0 {
            return;
        }

        // filled cells
        if !board.down(self.cell1).empty() || !board.down(self.cell2).empty() {
            return;
        }

        self.cell1.y -= 1;
        self.cell2.y -= 1;
    }

    fn hard_drop(&mut self, board: &Board) {
        loop {
            if self.fix(board) {
                break;
            }
            self.drop(board);
        }
    }

    fn clock_rotation(&mut self, board: &Board) {
        // placed hortizaontally
        if self.cell1.y == self.cell2.y {
            if self.cell2.x < self.cell1.x {
                std::mem::swap(&mut self.cell1, &mut self.cell2);
            }
            // self.cell1 is the left cell

            // rotate over left cell
            if self.cell1.y > 0 && board.down(self.cell1).empty() {
                self.cell2 = self.cell1.down();
            }
            // rotate over right cell
            if self.cell2.y < board.height - 1 && board.up(self.cell2).empty() {
                self.cell1 = self.cell2.up();
            }
        // placed vertically
        } else if self.cell1.x == self.cell2.x {
            if self.cell2.y > self.cell1.y {
                std::mem::swap(&mut self.cell1, &mut self.cell2);
            }
            // self.cell1 is the top cell

            // rotatre over bottom cell
            if self.cell2.x < board.width - 1 && board.right(self.cell2).empty() {
                self.cell1 = self.cell2.right();
            }
            // rotate top over cell
            else if self.cell1.x > 0 && board.left(self.cell1).empty() {
                self.cell2 = self.cell1.left();
            }
        }
    }

    fn anticlock_rotation(&mut self, board: &Board) {
        // placed hortizaontally
        if self.cell1.y == self.cell2.y {
            if self.cell2.x < self.cell1.x {
                std::mem::swap(&mut self.cell1, &mut self.cell2);
            }
            // self.cell1 is the left cell

            // rotate over right cell
            if self.cell2.y > 0 && board.down(self.cell2).empty() {
                self.cell1 = self.cell2.down();
            }
            // rotate over left cell
            if self.cell1.y < board.height - 1 && board.up(self.cell1).empty() {
                self.cell2 = self.cell1.up();
            }
        // placed vertically
        } else if self.cell1.x == self.cell2.x {
            if self.cell2.y > self.cell1.y {
                std::mem::swap(&mut self.cell1, &mut self.cell2);
            }
            // self.cell1 is the top cell

            // rotatre over bottom cell
            if self.cell2.x > 0 && board.left(self.cell2).empty() {
                self.cell1 = self.cell2.left();
            }
            // rotate top over cell
            else if self.cell1.x < board.width - 1 && board.right(self.cell1).empty() {
                self.cell2 = self.cell1.right();
            }
        }
    }

    fn fix(&self, board: &Board) -> bool {
        // right side
        if self.cell1.y == 0 || self.cell2.y == 0 {
            return true;
        }

        // filled cells
        if !board.down(self.cell1).empty() || !board.down(self.cell2).empty() {
            return true;
        }

        false
    }
}
