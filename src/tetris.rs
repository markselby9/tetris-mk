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

type ShapeData = Vec<(i32, i32)>;

struct Shape {
    data: ShapeData,
    height: usize,
}

fn build_shape(data: ShapeData, height: usize) -> Shape {
    Shape {
        data,
        height,
    }
}

pub enum ShapeType {
    // consider the top-center point as the bottom-center of the shape
    Square,
    S,
    Z,
    T,
    L,
    Line,
    MirroredL,
}

fn get_shape(shape_type: ShapeType) -> Shape {
    match shape_type {
        ShapeType::Square => build_shape(vec![(0, 0), (0, 1), (1, 0), (1, 1)], 2),
        ShapeType::S => build_shape(vec![(0, 0), (0, 1), (1, -1), (1, 0)], 2),
        _ => build_shape(vec![], 0),
    }
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

    fn add_shape(&mut self, shape: Shape) {
        // add a shape into the board, which should appear in the middle of the top row
        let mut next = self.cells.clone();
        for (delta_x, delta_y) in shape.data {
            next[(0 + delta_x) as usize][((self.width as i32 - 1) / 2 + delta_y) as usize]
                = Cell::Running;
        }
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
    fn test_add_shape_square() {
        let mut board = Board::new(8, 10).unwrap();
        board.add_shape(get_shape(ShapeType::Square));
        assert_eq!(*board.get_cell(0, 3), Cell::Running);
        assert_eq!(*board.get_cell(0, 4), Cell::Running);
        assert_eq!(*board.get_cell(1, 3), Cell::Running);
        assert_eq!(*board.get_cell(1, 4), Cell::Running);
    }

    #[test]
    fn test_add_shape_s() {
        let mut board = Board::new(8, 10).unwrap();
        board.add_shape(get_shape(ShapeType::S));
        println!("{}", board);
        assert_eq!(*board.get_cell(0, 3), Cell::Running);
        assert_eq!(*board.get_cell(0, 4), Cell::Running);
        assert_eq!(*board.get_cell(1, 2), Cell::Running);
        assert_eq!(*board.get_cell(1, 3), Cell::Running);
        assert_eq!(*board.get_cell(1, 4), Cell::Empty);
    }
}