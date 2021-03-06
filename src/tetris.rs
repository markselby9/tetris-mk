use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;
use wasm_bindgen::prelude::*;

// Error
#[derive(Debug, PartialEq)]
pub enum TetrisError {
    InvalidParam,
    InvalidIndex,
}

// Tetris board
#[wasm_bindgen]
#[repr(u8)] // 1 byte
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Running = 1,
    // for object which is dropping
    Placed = 2, // already dropped cell
}

// a vector of positions
type TransitionSet = Vec<(i32, i32)>;
type PositionSet = Vec<(usize, usize)>;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Shape {
    data: TransitionSet,
    pub(crate) shape_type: ShapeType,
    pub(crate) height: usize,
    pub(crate) running_cells: PositionSet,
    pub(crate) top_left_offset: TransitionSet,
}

fn build_shape(
    data: TransitionSet,
    height: usize,
    shape_type: ShapeType,
    top_left_offset: TransitionSet,
) -> Shape {
    Shape {
        data,
        height,
        shape_type,
        running_cells: vec![],
        top_left_offset,
    }
}

#[wasm_bindgen]
#[repr(u8)] // 1 byte
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ShapeType {
    // consider the top-center point as the bottom-center of the shape
    Square,
    S,
    Z,
    T,
    L,
    Line,
    MirroredL,
    Random,
}

// for random value from ShapeType enum
impl Distribution<ShapeType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ShapeType {
        match rng.gen_range(0, 6) {
            0 => ShapeType::Square,
            1 => ShapeType::S,
            2 => ShapeType::Z,
            3 => ShapeType::T,
            4 => ShapeType::L,
            5 => ShapeType::Line,
            6 => ShapeType::MirroredL,
            _ => ShapeType::Square,
        }
    }
}

pub fn generate_shape(shape_type: ShapeType) -> Shape {
    match shape_type {
        ShapeType::Square => build_shape(
            vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            2,
            ShapeType::Square,
            vec![(0, 0)],
        ),
        ShapeType::S => build_shape(
            vec![(0, 0), (0, 1), (1, -1), (1, 0)],
            3,
            ShapeType::S,
            vec![(-1, -1), (0, -1), (0, -1), (0, 0)],
        ),
        ShapeType::Z => build_shape(
            vec![(0, 0), (0, -1), (1, 0), (1, 1)],
            3,
            ShapeType::Z,
            vec![(-1, 0), (0, -2), (0, 0), (0, -1)],
        ),
        ShapeType::T => build_shape(
            vec![(0, 0), (0, -1), (0, 1), (1, 0)],
            3,
            ShapeType::T,
            vec![(-1, 0), (0, -1), (0, -1), (0, -1)],
        ),
        ShapeType::L => build_shape(
            vec![(0, -1), (1, -1), (1, 0), (1, 1)],
            3,
            ShapeType::L,
            vec![(0, 0), (0, -1), (-1, 0), (0, -1)],
        ),
        ShapeType::Line => build_shape(
            vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            4,
            ShapeType::Line,
            vec![(0, -2), (-2, 0), (0, -1), (-1, 0)],
        ),
        ShapeType::MirroredL => build_shape(
            vec![(0, 1), (1, 1), (1, 0), (1, -1)],
            3,
            ShapeType::MirroredL,
            vec![(0, -2), (0, 0), (-1, 0), (0, -1)],
        ),
        _ => generate_shape(rand::random::<ShapeType>()),
    }
}

#[wasm_bindgen]
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
    running_shape: Shape,
    next_shape_type: ShapeType,
    score: i32,
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        if width < 5 || width > 20 || height < 10 || height > 100 {
            panic!(TetrisError::InvalidParam);
        }
        let cells = vec![vec![Cell::Empty; width]; height];
        let score = 0;
        let running_shape = generate_shape(ShapeType::Random);
        let next_shape_type = ShapeType::Random;
        Board {
            width,
            height,
            cells,
            running_shape,
            next_shape_type,
            score,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn get_next_shape_type(&self) -> ShapeType {
        self.next_shape_type
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Board {
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

    pub fn set_score(&mut self, score: i32) {
        self.score = score;
    }

    pub fn get_running_shape(&self) -> &Shape {
        &self.running_shape
    }

    pub fn set_running_shape(&mut self, new_running_shape: Shape) {
        self.running_shape = new_running_shape;
    }

    pub fn get_running_cells(&self) -> &PositionSet {
        &self.get_running_shape().running_cells
    }

    pub fn set_running_cells(&mut self, new_running_cells: PositionSet) {
        self.running_shape.running_cells = new_running_cells;
    }

    pub fn set_next_shape_type(&mut self, new_next_shape_type: ShapeType) {
        self.next_shape_type = new_next_shape_type;
    }

    pub fn move_top_left_offset_array(&mut self) {
        let mut next_top_left_offset = self.running_shape.top_left_offset.clone();
        let first = next_top_left_offset.remove(0);
        next_top_left_offset.push(first);
        self.running_shape.top_left_offset = next_top_left_offset;
    }

    pub fn add_shape(&mut self, shape: Shape) {
        // add a shape into the board, which should appear in the middle of the top row

        let mut next = self.cells.clone();
        let mut next_running_cells: PositionSet = vec![];
        for (delta_x, delta_y) in &shape.data {
            let x = *delta_x as usize;
            let y = ((self.width as i32 - 1) / 2 + *delta_y) as usize;
            next_running_cells.push((x, y));
            next[x][y] = Cell::Running;
        }
        self.set_running_shape(shape);
        self.set_running_cells(next_running_cells);
        self.cells = next;
    }

    // to check whether one row or column cells are all in one state
    pub fn is_ith_column_all(&self, i: usize, state: Cell) -> bool {
        self.cells[i].iter().all(|&x| x == state)
    }

    pub fn is_ith_column_none(&self, i: usize, state: Cell) -> bool {
        self.cells[i].iter().all(|&x| x != state)
    }

    pub fn is_ith_row_all(&self, i: usize, state: Cell) -> bool {
        self.cells.iter().all(|column| column[i] == state)
    }

    pub fn is_ith_row_none(&self, i: usize, state: Cell) -> bool {
        self.cells.iter().all(|column| column[i] != state)
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
            writeln!(f);
        }
        Ok(()) // success result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_init() {
        //        let board = Board::new(1, 1);
        //        assert!(board.is_err(), "Invalid param");
        let board = Board::new(10, 30);
        assert_eq!(board.height, 30);
    }

    #[test]
    fn test_add_shape_square() {
        let mut board = Board::new(8, 10);
        board.add_shape(generate_shape(ShapeType::Square));
        assert_eq!(*board.get_cell(0, 3), Cell::Running);
        assert_eq!(*board.get_cell(0, 4), Cell::Running);
        assert_eq!(*board.get_cell(1, 3), Cell::Running);
        assert_eq!(*board.get_cell(1, 4), Cell::Running);
    }

    #[test]
    fn test_add_shape_s() {
        let mut board = Board::new(8, 10);
        board.add_shape(generate_shape(ShapeType::S));
        assert_eq!(*board.get_cell(0, 3), Cell::Running);
        assert_eq!(*board.get_cell(0, 4), Cell::Running);
        assert_eq!(*board.get_cell(1, 2), Cell::Running);
        assert_eq!(*board.get_cell(1, 3), Cell::Running);
        assert_eq!(*board.get_cell(1, 4), Cell::Empty);
    }
}
