use crate::tetris::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

//extern crate web_sys;

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
                let deleted_row_count = self.check_delete_rows() as i32;
                self.set_score(self.get_score() + deleted_row_count);
                //                add a random shape
                self.add_shape(get_shape(ShapeType::Random));
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
                    let mut can_move_left = true;
                    for (i, j) in self.get_running_cells().iter().cloned() {
                        if next[i][j - 1] == Cell::Placed {
                            can_move_left = false;
                            break;
                        }
                        next[i][j] = Cell::Empty;
                        new_running_cells.push((i, j - 1))
                    }
                    if !can_move_left {
                        return;
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
                    let mut can_move_right = true;
                    // can move right
                    for (i, j) in self.get_running_cells().iter().cloned() {
                        if next[i][j + 1] == Cell::Placed {
                            can_move_right = false;
                            break;
                        }
                        next[i][j] = Cell::Empty;
                        new_running_cells.push((i, j + 1))
                    }
                    if !can_move_right {
                        return;
                    }
                    for (i, j) in new_running_cells.iter().cloned() {
                        next[i][j] = Cell::Running;
                    }

                    self.set_cells(next);
                    self.set_running_cells(new_running_cells);
                }
            }
            Direction::Down => while self.drop() {},
        }
    }

    pub fn rotate(&mut self) {
        // https://strategywiki.org/wiki/Tetris/Rotation_systems#Nintendo_rotation_system
        // rotate by setting running cells

        if self.get_running_cells().is_empty() {
            return;
        }

        let mut next = self.get_cells().clone();
        let mut this_running_cells = self.get_running_cells().clone();
        let this_top_left_offset = &self.get_running_shape().top_left_offset.clone();
        this_running_cells.sort();
        console::log_1(&JsValue::from_str(&format!(
            "this_running_cells {:#?}",
            this_running_cells
        )));

        let mut top_left_point = (0, 0);
        let height = self.get_running_shape().height;

        //        find the position of top left point, in a 3*3 or 4*4 grid of the shape
        let offset = this_top_left_offset.get(0).unwrap();
        let offset_0 = ((offset.0 + self.get_height() as i32) as usize) % self.get_height();
        let offset_1 = ((offset.1 + self.get_width() as i32) as usize) % self.get_width();
        top_left_point.0 = (this_running_cells[0].0 + offset_0) % self.get_height();
        top_left_point.1 = (this_running_cells[0].1 + offset_1) % self.get_width();

        let mut new_grids_after_rotation = vec![vec![Cell::Empty; height]; height];
        for i in top_left_point.0..top_left_point.0 + height {
            for j in top_left_point.1..top_left_point.1 + height {
                if i >= self.get_height() || j >= self.get_width() {
                    // cannot rotate
                    return;
                }
                if next[i][j] == Cell::Running {
                    new_grids_after_rotation[height - 1 - (j - top_left_point.1)]
                        [i - top_left_point.0] = Cell::Running;
                    next[i][j] = Cell::Empty;
                }
            }
        }
        let mut next_running_cells = vec![];

        let mut can_rotate = true;
        for i in 0..height {
            for j in 0..height {
                if new_grids_after_rotation[i][j] == Cell::Running {
                    let temp_x = top_left_point.0 + i;
                    let temp_y = top_left_point.1 + j;
                    if temp_x >= self.get_height()
                        || temp_y >= self.get_width()
                        || next[top_left_point.0 + i][top_left_point.1 + j] == Cell::Placed
                    {
                        can_rotate = false;
                        break;
                    }
                    next_running_cells.push((temp_x, temp_y));
                }
            }
        }
        if !can_rotate {
            return;
        }
        for (i, j) in next_running_cells.iter().cloned() {
            next[i][j] = Cell::Running;
        }

        self.move_top_left_offset_array(); // update offset
        self.set_cells(next);
        self.set_running_cells(next_running_cells);
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
        let mut current = self.get_cells().clone();

        current = current
            .into_iter()
            .filter(|row| row.iter().any(|&x| x != Cell::Placed))
            .collect();
        let deleted_rows = self.get_cells().len() - current.len();

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
        let mut board = Board::new(8, 10);
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
        let mut board = Board::new(8, 10);
        for i in 0..8 {
            board.set_cell(3, i, Cell::Placed);
        }
        board.add_shape(get_shape(ShapeType::Square));
        assert_eq!(board.drop(), true);
        assert_eq!(board.drop(), false); // cannot drop here
    }

    #[test]
    fn test_tick_will_end() {
        let mut board = Board::new(8, 10);
        board.add_shape(get_shape(ShapeType::Square));
        //        board.set_next_shape_type(ShapeType::Square);
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
        let mut board = Board::new(8, 10);
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
        let mut board = Board::new(8, 10);
        board.add_shape(get_shape(ShapeType::Square));
        //        board.set_next_shape_type(ShapeType::Square);
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

        // test whether all deleted
        assert!(board.is_ith_column_all(board.get_height() - 1, Cell::Empty));
        assert!(board.is_ith_column_all(board.get_height() - 2, Cell::Empty));
    }

    #[test]
    fn test_rotate_line() {
        let mut board = Board::new(8, 10);
        board.set_running_shape(get_shape(ShapeType::Line));
        //        board.set_next_shape_type(ShapeType::Line);
        board.tick();
        board.tick();

        assert_eq!(*board.get_cell(1, 3), Cell::Running);
        board.rotate();
        assert_eq!(*board.get_cell(1, 3), Cell::Empty);
        assert_eq!(*board.get_cell(3, 1), Cell::Running);
        board.rotate();
        assert_eq!(*board.get_cell(1, 2), Cell::Running);
        assert_eq!(*board.get_cell(3, 1), Cell::Empty);
        board.rotate();
        println!("{}", board);
    }

    #[test]
    fn test_rotate_other_shape() {
        fn test_rotate_same(shape: ShapeType) {
            let mut board = Board::new(8, 10);
            board.set_running_shape(get_shape(ShapeType::MirroredL));
            //            board.set_next_shape_type(ShapeType::MirroredL);
            board.tick();
            board.tick();

            let mut running_cells_1 = board.get_running_cells().clone();
            board.rotate();
            board.rotate();
            board.rotate();
            board.rotate();
            let mut running_cells_2 = board.get_running_cells().clone();
            running_cells_1.sort();
            running_cells_2.sort();
            assert_eq!(running_cells_1, running_cells_2);
        }

        test_rotate_same(ShapeType::MirroredL);
        test_rotate_same(ShapeType::Z);
        test_rotate_same(ShapeType::S);
        test_rotate_same(ShapeType::Line);
        test_rotate_same(ShapeType::Square);
        test_rotate_same(ShapeType::L);
    }
}
