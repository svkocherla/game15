mod board;

use board::Board;

fn main() {
    println!("Program Start");

    let mut board = Board::new();
    println!("Displaying initial board");
    println!("{}", board.display());

    println!("Moving Left");
    board.move_left();
    println!("{}", board.display());

    println!("Moving Left");
    board.move_left();
    println!("{}", board.display());

    println!("Moving Left");
    board.move_left();
    println!("{}", board.display());

    println!("Moving Left");
    board.move_left();
    println!("{}", board.display());


    println!("Moving Right");
    board.move_right();
    println!("{}", board.display());

    println!("Moving Right");
    board.move_right();
    println!("{}", board.display());


    println!("Moving Up");
    board.move_up();
    println!("{}", board.display());


    println!("Moving Down");
    board.move_down();
    println!("{}", board.display());

    println!("Moving Down");
    board.move_down();
    println!("{}", board.display());

}
