use cgof2::cgof::Board;
use std::{thread, time};

fn main() {
    let mut board = Board::new(30);

    loop {
        println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
        board.print();
        board.step();
        thread::sleep(time::Duration::from_millis(75));
    }
}
