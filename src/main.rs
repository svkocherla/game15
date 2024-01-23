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

    println!("Checking Completion");
    println!("{}", board.is_complete());

    println!("Moving Right");
    board.move_right();
    println!("{}", board.display());

    println!("Checking Completion");
    println!("{}", board.is_complete());

    println!("Shuffling Board Randomly 10 times");
    board.shuffle(10);
    println!("{}", board.display());
}
