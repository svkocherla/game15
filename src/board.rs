use rand::Rng;

// 4 by 4 board
pub struct Board {
    grid: [[usize; 4]; 4],
    empty_location: (usize, usize),
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
        let empty_location = (3,3);

        Board { grid, empty_location }
    }

    #[allow(dead_code)]
    pub fn display(&self) -> String {
        let mut curr = String::new();
        for row in &self.grid {
            for &num in row {
                if num == 0 {
                    curr.push_str("   ");
                } else {
                    curr.push_str(&format!("{:3}", num));
                }
            }
            curr.push('\n');
        }

        curr
    }


    // getters for board for testing
    #[allow(dead_code)]
    pub fn get_grid(&self) -> &[[usize; 4]; 4] {
        &self.grid
    }

    #[allow(dead_code)]
    pub fn get_empty_location(&self) -> &(usize, usize) {
        &self.empty_location
    }

    // Shuffles the board to create random orientation
    #[allow(dead_code)]
    pub fn shuffle(&mut self, moves: usize) {
        let mut rng = rand::thread_rng();
        for _ in 0..moves {
            let rng_move = rng.gen::<usize>() % 4;

            match rng_move {
                0 => self.move_left(),
                1 => self.move_right(),
                2 => self.move_up(),
                3 => self.move_down(),
                _ => unreachable!(), // cause rust is angry at me
            }
        }
    }

    #[allow(dead_code)]
    pub fn is_complete(&self) -> bool {
        for row in 0..4 {
            for col in 0..4 {
                if row == 3 && col == 3 {
                    if self.grid[row][col] == 0 {
                        return true
                    } else {
                        return false
                    }
                }
                if self.grid[row][col] != row * 4 + (col + 1) {
                    return false
                }
            }
        }
        true
    }

    // moves perform a move on the grid if possible, else do nothing
    pub fn move_left(&mut self) {
        let (row, col) = self.empty_location;

        // cannot move left
        if col == 0 {
            return
        }

        let tmp = self.grid[row][col - 1];
        self.grid[row][col - 1] = 0;
        self.grid[row][col] = tmp;

        self.empty_location = (row, col - 1);
    }

    pub fn move_right(&mut self) {
        let (row, col) = self.empty_location;

        // cannot move right
        if col == 3 {
            return
        }

        let tmp = self.grid[row][col + 1];
        self.grid[row][col + 1] = 0;
        self.grid[row][col] = tmp;

        self.empty_location = (row, col + 1);
    }

    pub fn move_up(&mut self) {
        let (row, col) = self.empty_location;

        // cannot move up
        if row == 0 {
            return
        }

        let tmp = self.grid[row - 1][col];
        self.grid[row - 1][col] = 0;
        self.grid[row][col] = tmp;

        self.empty_location = (row - 1, col);
    }

    pub fn move_down(&mut self) {
        let (row, col) = self.empty_location;

        // cannot move down
        if row == 3 {
            return
        }

        let tmp = self.grid[row + 1][col];
        self.grid[row + 1][col] = 0;
        self.grid[row][col] = tmp;

        self.empty_location = (row + 1, col);
    }
}