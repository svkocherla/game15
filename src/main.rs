use std::io;

mod board;
use board::Board;

fn main() {
    println!("15 Puzzle!");
    println!("Use wasd to move and q to quit");

    // create grid and shuffle
    const SHUFFLE_CONSTANT: usize = 200;
    let mut board = Board::new();
    while board.is_complete() {
        board.shuffle(SHUFFLE_CONSTANT);
    }

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
            _ => println!("You did not enter wasd or q!"),
        }

        input.clear();
        print!("{}", board.display());

        if board.is_complete() {
            println!("You solved the puzzle");
            break;
        }

        println!("Enter wasd or q!");
    }

    println!("Thanks for playing");
}
