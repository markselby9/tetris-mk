use crate::tetris::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)] // 1 byte
pub enum Direction {
    Left = 0,
    Right = 1,
    Down = 2,
}

#[wasm_bindgen]
impl Board {
    // return false if game over, true if continues
    pub fn tick(&mut self) -> bool {
        //  what happens in the next frame
        let try_drop = self.drop();
        if !try_drop {
            //todo: check whether game is already over
            if !self.is_ith_column_all(0, Cell::Empty) {
                return false;
            } else {
                self.check_delete_rows();
                self.add_shape(get_shape(ShapeType::Square));
            }
        }
        true
    }

    pub fn move_shape(&mut self, direction: Direction) {
        let mut next = self.get_cells().clone();
        let mut new_running_cells = vec![];
        match direction {
            Direction::Left => {
                if self.is_ith_row_none(0, Cell::Running) {
                    // can move left
                    for (i, j) in self.get_running_cells().iter().cloned() {
                        next[i][j] = Cell::Empty;
                        new_running_cells.push((i, j - 1))
                    }
                    for (i, j) in new_running_cells.iter().cloned() {
                        next[i][j] = Cell::Running;
                    }

                    self.set_cells(next);
                    self.set_running_cells(new_running_cells);
                }
            }
            Direction::Right => {
                if self.is_ith_row_none(self.get_width() - 1, Cell::Running) {
                    // can move left
                    for (i, j) in self.get_running_cells().iter().cloned() {
                        next[i][j] = Cell::Empty;
                        new_running_cells.push((i, j + 1))
                    }
                    for (i, j) in new_running_cells.iter().cloned() {
                        next[i][j] = Cell::Running;
                    }

                    self.set_cells(next);
                    self.set_running_cells(new_running_cells);
                }
            }
            Direction::Down => {
                while self.drop() {
                    println!("{}", self)
                }
            }
        }
    }
}

impl Board {
    fn drop(&mut self) -> bool {
        let mut next = self.get_cells().clone();
        let mut new_running_cells = vec![];

        if self.get_running_cells().is_empty() {
            return false;
        }

        // check whether can drop
        for (i, j) in self.get_running_cells().iter().cloned() {
            if i == self.get_height() - 1 || next[i + 1][j] == Cell::Placed {
                // cannot drop, turn all the shape into Placed
                {
                    for m in 0..self.get_height() {
                        for n in 0..self.get_width() {
                            if next[m][n] == Cell::Running {
                                next[m][n] = Cell::Placed;
                            }
                        }
                    }
                }
                self.set_cells(next);
                self.set_running_cells(new_running_cells);
                // then return false
                return false;
            }
        }

        for (i, j) in self.get_running_cells().iter().cloned() {
            if next[i][j] == Cell::Running {
                next[i][j] = Cell::Empty;
                new_running_cells.push((i + 1, j));
            }
        }
        for (i, j) in new_running_cells.iter().cloned() {
            next[i][j] = Cell::Running;
        }
        self.set_cells(next);
        self.set_running_cells(new_running_cells);
        true
    }

    fn check_delete_rows(&mut self) -> usize {
        let mut deleted_rows: usize = 0;
        let mut current = self.get_cells().clone();

        while !current.is_empty() && current.last().unwrap().iter().all(|&x| x == Cell::Placed) {
            // remove this column
            current.pop();
            deleted_rows += 1;
        }
        if deleted_rows > 0 {
            let mut next = vec![vec![Cell::Empty; self.get_width()]; deleted_rows];
            next.append(&mut current);
            self.set_cells(next);
        }
        deleted_rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_tick_will_end() {
        let mut board = Board::new(8, 10).unwrap();
        board.add_shape(get_shape(ShapeType::Square));
        let mut count = 0;

        loop {
            if !board.tick() {
                break;
            }
            count += 1;
            if count == 60 {
                assert!(false)
            }
        }
        assert!(true);
    }

    #[test]
    fn test_move_shape() {
        let mut board = Board::new(8, 10).unwrap();
        board.add_shape(get_shape(ShapeType::Square));

        board.move_shape(Direction::Left);
        assert_eq!(*board.get_cell(1, 2), Cell::Running);
        assert_eq!(*board.get_cell(1, 3), Cell::Running);
        assert_eq!(*board.get_cell(1, 4), Cell::Empty);
        assert_eq!(*board.get_cell(1, 5), Cell::Empty);

        board.move_shape(Direction::Right);
        board.move_shape(Direction::Right);
        assert_eq!(*board.get_cell(1, 4), Cell::Running);
        assert_eq!(*board.get_cell(1, 7), Cell::Empty);
        board.move_shape(Direction::Down);

        assert_eq!(*board.get_cell(8, 2), Cell::Empty);
        assert_eq!(*board.get_cell(8, 3), Cell::Empty);
        assert_eq!(*board.get_cell(8, 4), Cell::Placed);
        assert_eq!(*board.get_cell(8, 5), Cell::Placed);
    }

    #[test]
    fn test_check_delete_rows() {
        //        test in a integrated way
        let mut board = Board::new(8, 10).unwrap();
        board.add_shape(get_shape(ShapeType::Square));
        board.move_shape(Direction::Left);
        board.move_shape(Direction::Left);
        board.move_shape(Direction::Left);
        board.move_shape(Direction::Left);
        board.move_shape(Direction::Down);

        board.tick();
        board.move_shape(Direction::Left);
        board.move_shape(Direction::Down);
        board.tick();
        board.move_shape(Direction::Right);
        board.move_shape(Direction::Down);
        board.tick();

        board.move_shape(Direction::Right);
        board.move_shape(Direction::Right);
        board.move_shape(Direction::Right);
        board.move_shape(Direction::Down);
        board.tick();

        println!("{}", board);
        // test whether all deleted
        assert!(board.is_ith_column_all(board.get_height() - 1, Cell::Empty));
        assert!(board.is_ith_column_all(board.get_height() - 2, Cell::Empty));
    }
}
