use conways_game_of_life::cgof::Board;
use std::{env, thread, time};

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut board = Board::new(args[1].parse().unwrap_or(30));

    loop {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        board.print();
        board.step();
        thread::sleep(time::Duration::from_millis(args[2].parse().unwrap_or(75)));
    }
}
