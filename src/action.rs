use crate::tetris::*;

pub enum Direction {
    Left = 0,
    Right = 1,
    Down = 2,
}

impl Board {
    fn drop(&mut self) -> bool {
        let mut next = self.get_cells().clone();
        for i in (0..self.get_height()).rev() {
            for j in 0..self.get_width() {
                if next[i][j] == Cell::Running {
                    // try to drop this cell
                    if i == self.get_height() - 1 || next[i + 1][j] != Cell::Empty {
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
                        // then return false
                        return false;
                    }
                    next[i + 1][j] = Cell::Running;
                    next[i][j] = Cell::Empty;
                }
            }
        }
        self.set_cells(next);
        true
    }

    fn move_shape(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                println!("left");
            }
            Direction::Right => {
                println!("right");
            }
            Direction::Down => {
                while self.drop() {}
            }
        }
    }

    // return false if game over, true if continues
    pub(crate) fn tick(&mut self) -> bool {
        //  what happens in the next frame
        let try_drop = self.drop();
        if !try_drop {
            //todo: check whether game is already over
            if !self.is_ith_column_empty(0) {
                return false;
            } else {
                self.add_shape(get_shape(ShapeType::Square));
            }
        }
        true
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
        board.move_shape(Direction::Right);
        board.move_shape(Direction::Down);

        println!("{}", board);
        assert_eq!(*board.get_cell(8, 2), Cell::Empty);
        assert_eq!(*board.get_cell(8, 3), Cell::Placed);
        assert_eq!(*board.get_cell(8, 4), Cell::Placed);
        assert_eq!(*board.get_cell(8, 5), Cell::Empty);
    }
}
