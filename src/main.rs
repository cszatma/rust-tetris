use std::io;
use std::io::BufRead;
mod game;
mod tetrimino;

use tetrimino::Tetrimino;

fn main() {
    //let mut input = Vec::new();
    let mut input:Vec<Vec<String>> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        //input.push(line.unwrap());
        input.push(line.unwrap().split(' ').map(|s| s.to_string()).collect());
    }
    println!("{:?}", input);
}
