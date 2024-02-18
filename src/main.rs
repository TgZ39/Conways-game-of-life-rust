use clap::Parser;
use conways_game_of_life::cgof::Board;
use std::{thread, time};

/// Conway's game of life [rust implementation]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Length of the emulation board
    #[arg(short, long, default_value_t = 20)]
    board_length: u32,

    /// Delay before new iteration of board is displayed. (in ms)
    #[arg(short, long, default_value_t = 100)]
    delay_millis: u64,
}

fn main() {
    let args = Args::parse();
    let mut board = Board::new(args.board_length);

    loop {
        // println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"); keeping this for the memes
        clearscreen::clear().unwrap();
        board.print();
        board.step();
        thread::sleep(time::Duration::from_millis(args.delay_millis));
    }
}
