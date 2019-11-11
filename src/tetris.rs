use std::io;
use std::fmt;
//use wasm_bindgen::prelude::*;

// Error
#[derive(Debug, PartialEq)]
pub enum TetrisError {
    InvalidParam,
    InvalidIndex,
}

// Tetris board
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Running = 1,
    // for object which is dropping
    Placed = 2,  // already dropped cell
}

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Result<Board, TetrisError> {
        if width < 5 || width > 20 || height < 10 || height > 100 {
            return Err(TetrisError::InvalidParam);
        }
        let mut cells = vec![vec![Cell::Empty; width]; height];
        Ok(Board {
            width,
            height,
            cells,
        })
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[x][y]
    }

    fn add_shape(&mut self) {
        // add a shape into the board, which should appear in the middle of the top row
        let mut next = self.cells.clone();
        next[0][self.width / 2] = Cell::Running;
        self.cells = next;
    }

    fn tick() {
        //  what happens in the next frame
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice() {
            for &pos in line.as_slice() {
                let symbol = if pos == Cell::Placed || pos == Cell::Running { "|x|" } else { "| |" };
                write!(f, "{}", symbol);
            }
            write!(f, "\n");
        }
        Ok(())  // success result
    }
}


// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_init() {
        let board = Board::new(1, 1);
        assert!(board.is_err(), "Invalid param");
        let board = Board::new(10, 30);
        assert_eq!(board.unwrap().height, 30);
    }

    #[test]
    fn test_add_shape() {
        let mut board = Board::new(7, 10).unwrap();
        board.add_shape();
        println!("{}", board);
        assert_eq!(*board.get_cell(0, board.width / 2), Cell::Running);
    }
}