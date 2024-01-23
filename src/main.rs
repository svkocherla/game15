use std::io;

mod board;
use board::Board;

fn main() {
    println!("15 Puzzle!");
    println!("Use wasd to move and q to quit");

    // create grid and shuffle
    const SHUFFLE_CONSTANT: usize = 1;
    let mut board = Board::new();
    board.shuffle(SHUFFLE_CONSTANT);

    println!("Initial Board");
    print!("{}", board.display());


    
    println!("Enter wasd or q!");

    let mut input = String::new();
    while input.trim() != "q" {
        io::stdin()
            .read_line(&mut input)
            .expect("Error occured");
        
        match input.trim() {
            //instructions are reversed
            "w" => board.move_down(),
            "a" => board.move_right(),
            "s" => board.move_up(),
            "d" => board.move_left(),
            "q" => break,
            _ => println!("You did not enter WASD or Q!"),
        }

        input.clear();
        print!("{}", board.display());

        if board.is_complete() {
            println!("You solved the puzzle");
            break;
        }

        println!("Enter WASD or Q!");
    }

    println!("Thanks for playing");
}
