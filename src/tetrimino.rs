pub struct Tetrimino {
    points: Vec<Vec<i32>>,
    position: (i32, i32),
    symbol: String,
}

impl Tetrimino {
    // Static methods for creating tetriminos
    pub fn create(type_num: i32, position: (i32, i32)) -> Tetrimino {
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
}