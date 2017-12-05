use std::io;
use std::io::BufRead;
use board::Board;
use tetrimino::Tetrimino;
use game::Game;

mod game;
mod tetrimino;
mod board;
mod status;

fn main() {
    //let mut input = Vec::new();
    let mut input: Vec<Vec<String>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        //input.push(line.unwrap());
        input.push(line.unwrap().split(' ').map(|s| s.to_string()).collect());
    }
    println!("{:?}", input);
    let mut game = Game::init(input[0][1].parse().unwrap(), input[0][2].parse().unwrap());
    print_board(game.get_board())
}

fn print_board(board: &Board) {
    for row in board.values() {
        println!("|{}|", row.join(""));
    }
    let num_of_cols = board.num_of_cols();
    println!("+{}+", vec!["-"; num_of_cols as usize].join(""));
}
