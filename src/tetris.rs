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
    Placed = 2, // already dropped cell
}

type ShapeData = Vec<(i32, i32)>;

pub struct Shape {
    data: ShapeData,
    height: usize,
}

fn build_shape(data: ShapeData, height: usize) -> Shape {
    Shape { data, height }
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

pub fn get_shape(shape_type: ShapeType) -> Shape {
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
    pub fn new(width: usize, height: usize) -> Result<Board, TetrisError> {
        if width < 5 || width > 20 || height < 10 || height > 100 {
            return Err(TetrisError::InvalidParam);
        }
        let cells = vec![vec![Cell::Empty; width]; height];
        Ok(Board {
            width,
            height,
            cells,
        })
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_cells(&self) -> &Vec<Vec<Cell>> {
        &self.cells
    }

    pub fn set_cells(&mut self, new_cells: Vec<Vec<Cell>>) {
        self.cells = new_cells;
    }

    pub fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[x][y]
    }

    pub fn set_cell(&mut self, x: usize, y: usize, val: Cell) {
        self.cells[x][y] = val;
    }

    pub fn add_shape(&mut self, shape: Shape) {
        // add a shape into the board, which should appear in the middle of the top row
        let mut next = self.cells.clone();
        for (delta_x, delta_y) in shape.data {
            next[(0 + delta_x) as usize][((self.width as i32 - 1) / 2 + delta_y) as usize] =
                Cell::Running;
        }
        self.cells = next;
    }

    pub fn is_ith_column_empty(&self, i: usize) -> bool {
        self.cells[i].iter().all(|&x| x == Cell::Empty)
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
            writeln!(f, "");
        }
        Ok(()) // success result
    }
}

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
}
