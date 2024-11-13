use std::vec::Vec;

use crate::game::piece::domino::Domino;
use crate::game::piece::Piece;

#[derive(Clone, Copy)]
pub enum CellState {
    Empty,
    Filled,
}

#[derive(Clone, Copy)]
pub struct GridPosition {
    pub x: usize,
    pub y: usize,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> GridPosition {
        GridPosition {
            x: x as usize,
            y: y as usize,
        }
    }

    pub fn left(&self) -> GridPosition {
        GridPosition {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> GridPosition {
        GridPosition {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn up(&self) -> GridPosition {
        GridPosition {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn down(&self) -> GridPosition {
        GridPosition {
            x: self.x,
            y: self.y - 1,
        }
    }
}

impl From<(i32, i32)> for GridPosition {
    fn from(coords: (i32, i32)) -> Self {
        GridPosition::new(coords.0, coords.1)
    }
}

pub struct Cell {
    state: CellState,
    position: GridPosition,
}

impl Cell {
    fn new(position: GridPosition) -> Cell {
        Cell {
            state: CellState::Empty,
            position,
        }
    }

    pub fn state(&self) -> CellState {
        self.state
    }

    pub fn empty(&self) -> bool {
        matches!(self.state, CellState::Empty)
    }

    pub fn x(&self) -> usize {
        self.position.x
    }
    pub fn y(&self) -> usize {
        self.position.y
    }

    pub fn fill(&mut self) {
        self.state = CellState::Filled;
    }
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut cells: Vec<Vec<Cell>> = Vec::with_capacity(width);
        for col in 0..width {
            let mut row_vec: Vec<Cell> = Vec::with_capacity(height);
            for row in 0..height {
                row_vec.push(Cell::new(GridPosition::new(col as i32, row as i32)));
            }
            cells.push(row_vec);
        }

        Board {
            width,
            height,
            cells,
        }
    }

    pub fn get_cell(&self, pos: GridPosition) -> &Cell {
        &self.cells[pos.x][pos.y]
    }

    pub fn cell(&mut self, pos: GridPosition) -> &mut Cell {
        &mut self.cells[pos.x][pos.y]
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter().flat_map(|row| row.iter())
    }

    pub fn cells_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn up(&self, pos: GridPosition) -> &Cell {
        self.get_cell(pos.up())
    }

    pub fn down(&self, pos: GridPosition) -> &Cell {
        self.get_cell(pos.down())
    }

    pub fn left(&self, pos: GridPosition) -> &Cell {
        self.get_cell(pos.left())
    }

    pub fn right(&self, pos: GridPosition) -> &Cell {
        self.get_cell(pos.right())
    }

    fn fill(&mut self, pos: GridPosition) -> &mut Self {
        self.cell(pos).fill();
        self
    }

    pub fn merge(&mut self, piece: &Domino) -> &mut Self {
        for pos in piece.positions() {
            self.fill(pos);
        }
        self
    }

    pub fn clear_rows(&mut self) -> &mut Self {
        for y in (0..self.height).rev() {
            let mut full = true;
            // TODO: continue amb tags?
            for x in 0..self.width {
                if self.get_cell(GridPosition { x, y }).empty() {
                    full = false;
                    break;
                }
            }
            if !full {
                continue;
            }

            for row in y + 1..self.height {
                for x in 0..self.width {
                    self.cells[x][row - 1].state = self.cells[x][row].state();
                }
            }

            for x in 0..self.width {
                self.cells[x][self.height - 1].state = CellState::Empty;
            }
        }
        self
    }
}
