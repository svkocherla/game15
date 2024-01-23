mod board;

use board::Board;

fn main() {
    println!("Program Start");
    let board = Board::new();
    println!("Displaying initial board");
    board.display();
}
