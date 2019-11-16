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

    fn set_cell(&mut self, x: usize, y: usize, val: Cell) {
        self.cells[x][y] = val;
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
    
    fn drop(&mut self) -> bool {
        let mut next = self.cells.clone();
        for i in (0..self.height).rev() {
            for j in 0..self.width {
                if next[i][j] == Cell::Running {
                    // try to drop this cell
                    if i == self.height - 1 || next[i+1][j] != Cell::Empty {
                        //TODO: cannot drop, turn all the shape into Placed
                        return false;
                    }
                    next[i+1][j] = Cell::Running;
                    next[i][j] = Cell::Empty;
                }
            }
        }
        self.cells = next;
        return true;
    }

    fn move_shape(&mut self) {
    }

    fn is_ith_column_empty(&self, i:usize) -> bool {
        self.cells[i].iter().all(|&x| x == Cell::Empty)
    }

    fn tick(&mut self) {
        //  what happens in the next frame
        let try_drop = self.drop();
        if !try_drop {
            //todo: check whether game is already over
            if (!self.is_ith_column_empty(0)) {
                println!("GG!!");
            }
            else {
                self.add_shape(get_shape(ShapeType::Square));
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice() {
            for &pos in line.as_slice() {
                let symbol = match pos {
                    Cell::Placed => "|x|",
                    Cell::Running => "|*|",
                    _ => "| |",
                };
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
        assert_eq!(*board.get_cell(0, 3), Cell::Running);
        assert_eq!(*board.get_cell(0, 4), Cell::Running);
        assert_eq!(*board.get_cell(1, 2), Cell::Running);
        assert_eq!(*board.get_cell(1, 3), Cell::Running);
        assert_eq!(*board.get_cell(1, 4), Cell::Empty);
    }

    #[test]
    fn test_drop() {
        let mut board = Board::new(8, 10).unwrap();
        board.add_shape(get_shape(ShapeType::S));
        board.drop();
        board.drop();
        assert_eq!(*board.get_cell(1, 4), Cell::Empty);
        assert_eq!(*board.get_cell(2, 3), Cell::Running);
        assert_eq!(*board.get_cell(2, 4), Cell::Running);
        assert_eq!(*board.get_cell(3, 2), Cell::Running);
        assert_eq!(*board.get_cell(3, 3), Cell::Running);
        assert_eq!(*board.get_cell(3, 4), Cell::Empty);
    }

    #[test]
    fn test_cannot_drop() {
        let mut board = Board::new(8, 10).unwrap();
        for i in 0..8 {
            board.set_cell(3, i, Cell::Placed);
        }
        board.add_shape(get_shape(ShapeType::Square));
        assert_eq!(board.drop(), true);
        assert_eq!(board.drop(), false); // cannot drop here
    }

    #[test]
    fn test_tick() {
        let mut board = Board::new(8, 10).unwrap();
        board.add_shape(get_shape(ShapeType::Square));

        println!("{}", board);
        board.tick();
        board.tick();
        board.tick();
        board.tick();
        board.tick();
        println!("{}", board);
        board.tick();
        board.tick();
        board.tick();
        board.tick();
        println!("{}", board);
        board.tick();
        board.tick();
        board.tick();
        println!("{}", board);
    }
}
