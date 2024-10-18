use std::vec::Vec;

#[derive(Clone)]
enum CellState {
    Empty,
    Filled,
}

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellState>>,
}

pub fn new_board(width: usize, height: usize) -> Board {
    let cells = vec![vec![CellState::Empty; width]; height];
    Board {
        width,
        height,
        cells,
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let cells = vec![vec![CellState::Empty; width]; height];
        Board {
            width,
            height,
            cells,
        }
    }

    pub fn print(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    CellState::Empty => print!("-"),
                    CellState::Filled => print!("X"),
                }
            }
            println!();
        }
    }
}
