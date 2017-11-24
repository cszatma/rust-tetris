use tetrimino::Tetrimino;
use board::Board;

pub struct Game {
    board: Board,
    current_piece: Option<Tetrimino>,
}

impl Game {
    // Static methods
    pub fn init(num_rows: i32, num_cols: i32) -> Game {
        return Game { board: Board::init(num_rows, num_cols), current_piece: None }
    }
}