mod board;
use board::Board;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_move_left() {
        
        
        let mut board = Board::new();
        board.move_left();

        let expected = [
            [1,2,3,4],
            [5,6,7,8],
            [9,10,11,12],
            [13,14,0,15],
        ];

        assert_eq!(board.get_empty_location().0, 3);
        assert_eq!(board.get_empty_location().1, 2);
        assert_eq!(*board.get_grid(), expected);
    }

    #[test]
    fn test_move_right() {
        
        
        let mut board = Board::new();
        board.move_right();

        let expected = [
            [1,2,3,4],
            [5,6,7,8],
            [9,10,11,12],
            [13,14,15,0],
        ];

        assert_eq!(board.get_empty_location().0, 3);
        assert_eq!(board.get_empty_location().1, 3);
        assert_eq!(*board.get_grid(), expected);
    }

    #[test]
    fn test_move_left_right() {
        
        
        let mut board = Board::new();
        board.move_left();
        board.move_right();

        let expected = [
            [1,2,3,4],
            [5,6,7,8],
            [9,10,11,12],
            [13,14,15,0],
        ];

        assert_eq!(board.get_empty_location().0, 3);
        assert_eq!(board.get_empty_location().1, 3);
        assert_eq!(*board.get_grid(), expected);
    }

    #[test]
    fn test_move_up() {
        
        
        let mut board = Board::new();
        board.move_up();

        let expected = [
            [1,2,3,4],
            [5,6,7,8],
            [9,10,11,0],
            [13,14,15,12],
        ];

        assert_eq!(board.get_empty_location().0, 2);
        assert_eq!(board.get_empty_location().1, 3);
        assert_eq!(*board.get_grid(), expected);
    }

    #[test]
    fn test_move_down() {
        
        
        let mut board = Board::new();
        board.move_down();

        let expected = [
            [1,2,3,4],
            [5,6,7,8],
            [9,10,11,12],
            [13,14,15,0],
        ];

        assert_eq!(board.get_empty_location().0, 3);
        assert_eq!(board.get_empty_location().1, 3);
        assert_eq!(*board.get_grid(), expected);
    }

    #[test]
    fn test_move_up_down() {
        
        
        let mut board = Board::new();
        board.move_up();
        board.move_down();

        let expected = [
            [1,2,3,4],
            [5,6,7,8],
            [9,10,11,12],
            [13,14,15,0],
        ];

        assert_eq!(board.get_empty_location().0, 3);
        assert_eq!(board.get_empty_location().1, 3);
        assert_eq!(*board.get_grid(), expected);
    }
}