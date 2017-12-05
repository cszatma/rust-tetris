use board::Board;
use status::Status;

pub struct Tetrimino {
    points: Vec<Vec<i32>>,
    position: (i32, i32),
    symbol: String,
}

impl Tetrimino {
    // Static methods for creating tetriminos
    pub fn init(type_num: i32, position: (i32, i32)) -> Tetrimino {
        match type_num {
            1 => Tetrimino::type_one(position),
            2 => Tetrimino::type_two(position),
            3 => Tetrimino::type_three(position),
            4 => Tetrimino::type_four(position),
            5 => Tetrimino::type_five(position),
            6 => Tetrimino::type_six(position),
            7 => Tetrimino::type_seven(position),
            _ => panic!("Invalid tetrimino type: {}", type_num),
        }
    }

    fn type_one(position: (i32, i32)) -> Tetrimino {
        return Tetrimino { points: vec![vec![0, 0], vec![1, 0], vec![0, -1], vec![1, -1]], position: position, symbol: "y".to_string() };
    }


    fn type_two(position: (i32, i32)) -> Tetrimino {
        return Tetrimino { points: vec![vec![-1, -1], vec![0, -1], vec![0, 0], vec![1, 0]], position: position, symbol: "r".to_string() };
    }

    fn type_three(position: (i32, i32)) -> Tetrimino {
        return Tetrimino { points: vec![vec![-1, 0], vec![0, 0], vec![0, -1], vec![1, -1]], position: position, symbol: "g".to_string() };
    }

    fn type_four(position: (i32, i32)) -> Tetrimino {
        return Tetrimino { points: vec![vec![-1, -1], vec![-1, 0], vec![0, 0], vec![1, 0]], position: position, symbol: "b".to_string() };
    }

    fn type_five(position: (i32, i32)) -> Tetrimino {
        return Tetrimino { points: vec![vec![-1, 0], vec![0, 0], vec![1, 0], vec![1, -1]], position: position, symbol: "o".to_string() };
    }

    fn type_six(position: (i32, i32)) -> Tetrimino {
        return Tetrimino { points: vec![vec![-1, 0], vec![0, 0], vec![1, 0], vec![0, -1]], position: position, symbol: "p".to_string() };
    }

    fn type_seven(position: (i32, i32)) -> Tetrimino {
        return Tetrimino { points: vec![vec![-1, 0], vec![0, 0], vec![1, 0], vec![2, 0]], position: position, symbol: "c".to_string() };
    }

    // Instance methods
    pub fn get_pos_on_board(&self) -> Vec<Vec<i32>> {
        return self.points.iter().map(|point| vec![point[0] + self.position.0, point[1] + self.position.1]).collect();
    }

    pub fn get_x_values(&self) -> Vec<i32> {
        return self.get_pos_on_board().iter().map(|point| point[0]).collect();
    }

    pub fn get_y_values(&self) -> Vec<i32> {
        return self.get_pos_on_board().iter().map(|point| point[1]).collect();
    }

    // Moves
    pub fn perform_move(&mut self, board: &Board, move_str: String) -> Status {
        match move_str.as_str() {
            "l" | "r" => self.shift_horizontally(board, move_str),
            "."       => self.shift_down(board),
            "+"       => self.perform_drop(board),
            _         => panic!("Invalid move: {}", move_str),
        }
    }

    fn shift_horizontally(&mut self, board: &Board, direction: String) -> Status {
        let shift_value: i32;
        if direction == "l" { shift_value = -1; } else { shift_value = 1; }
        let (x, y) = self.position;
        self.position.0 += shift_value;
        if self.piece_does_overlap(board) {
            self.position.0 -= shift_value;
            return Status::Failed;
        }
        return Status::Ok;
    }

    fn shift_down(&mut self, board: &Board) -> Status {
        if self.is_on_bottom(board) { return Status::Placed; }
        self.position.1 += 1;
        if self.piece_does_overlap(board) {
            self.position.1 -= 1;
            return Status::Placed;
        }
        return Status::Ok;
    }

    fn perform_drop(&mut self, board: &Board) -> Status {
        let status = self.shift_down(board);
        if status == Status::Placed { return status; }
        return self.perform_drop(board);
    }

    fn is_on_bottom(&self, board: &Board) -> bool {
        return self.get_y_values().contains(&(board.num_of_cols() - 1));
    }

    fn piece_does_overlap(&self, board: &Board) -> bool {
        return board.has_piece_at_positions(self.get_pos_on_board());
    }

    fn is_out_of_bounds(&self, board: &Board) -> bool {
        let x_values = self.get_x_values();
        let num_of_rows = board.num_of_rows();
        return  x_values.iter().any(| x | x == &-1 || x == &num_of_rows);
    }
}