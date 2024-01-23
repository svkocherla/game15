// 4 by 4 board
pub struct Board {
    grid: [[usize; 4]; 4],
    // empty_location: (usize, usize),
}

impl Board {
    pub fn new() -> Board {
        let mut grid = [[0; 4]; 4];
        
        // initialize grid
        for row in 0..4 {
            for col in 0..4 {
                grid[row][col] = row * 4 + (col + 1);
            }
        }

        grid[3][3] = 0;
        // let mut empty_location = (3,3);

        Board { grid }
    }

    pub fn display(&self) {
        for row in &self.grid {
            for &num in row {
                if num == 0 {
                    print!("  #");
                } else {
                    print!("{:3}", num);
                }
            }
            println!()
        }
    }

    // moves perform a move on the grid if possible, else do nothing
    // pub fn move_left(&mut self) {

    // }

    // pub fn move_right(&mut self) {
        
    // }

    // pub fn move_up(&mut self) {
        
    // }

    // pub fn move_down(&mut self) {
        
    // }

    // pub fn shuffle(&mut self, moves: usize) {

    // }

    // pub fn is_complete(&self) -> bool {
    //     false
    // }
}