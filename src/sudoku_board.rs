// Sudoku Board Module
use std::collections::HashSet;

#[derive(Clone, Copy)]
pub struct SudokuBoard {
    board: [[u8; 9]; 9],
    initial_mask: [[bool; 9]; 9],
}

impl SudokuBoard {
    // Class Constructor
    // Assume config always exists for now.
    pub fn from(config: [[u8; 9]; 9]) -> Result<Self, &'static str> {
        if !Self::is_valid_config(&config) {
            return Err("Error: Invalid config used in SudokuBoard::from().");
        }

        let mut initial_mask = [[false; 9]; 9];
        for r in 0..9 {
            for c in 0..9 {
                if config[r][c] != 0 {
                    initial_mask[r][c] = true;
                }
            }
        }

        Ok(SudokuBoard {
            board: config,
            initial_mask,
        })
    }

    // Getters and Setters
    pub fn get(&self, cell: (u8, u8)) -> Option<u8> {
        // Validates that the cell is on the board and returns value if it is
        self.board
            .get(cell.0 as usize)?
            .get(cell.1 as usize)
            .copied()
    }

    pub fn set(&mut self, cell: (u8, u8), num: u8) -> Result<(), &'static str> {
        let (r, c) = (cell.0 as usize, cell.1 as usize);
        // Check if the cell is part of the initial configuration.
        if self.initial_mask[r][c] {
            return Err("Error: Cannot modify a starting number.");
        }
        // If not, proceed to place the number.
        if let Some(row) = self.board.get_mut(r) {
            if let Some(elem) = row.get_mut(c) {
                *elem = num;
                return Ok(())
            }
        }
        Err("Error: Invalid move.")
    }

    pub fn print(&self) {
        // Print the Sudoku Board in a human readable
        println!("{}", "-".repeat(31));
        println!("|     CURRENT BOARD STATE      |");
        println!("{}", "-".repeat(31));
        for (row_index, row) in self.board.iter().enumerate() {
            print!("|");
            for (col_index, &element) in row.iter().enumerate() {
                print!(" {} ", element);
                if (col_index + 1) % 3 == 0 {
                    print!("|")
                }
            }
            println!();
            if (row_index + 1) % 3 == 0 {
                println!("{}", "-".repeat(31));
            }
        }
    }

    // Check if a proposed move is legal
    pub fn validate_move(&self, cell: (u8, u8), num: u8) -> bool {
        let (r, c) = (cell.0 as usize, cell.1 as usize);

        // A move is invalid if the number is not 0-9
        if !(0..=9).contains(&num) {
            return false;
        }

        // A move is invalid if the cell is out of bounds
        if r >= 9 || c >= 9 {
            return false;
        }

        // A move is invalid if it modifies a starting number.
        if self.initial_mask[r][c] {
            return false;
        }
        // A move clearing a non-starting number cell is valid.
        if num == 0 {
            return true;
        }

        // A move overwriting a filled cell is invalid.
        if self.board[r][c] != 0 {
            return false;
        }

        // Check vertically for duplicates
        for row in &self.board {
            if row[c] == num {
                return false
            }
        }
        // Check horizontally for duplicates
        for &element in &self.board[r] {
            if element == num {
                return false
            }
        }

        // Calculate top-left corner of the 3x3 box.
        let box_r_start: usize = (r / 3) * 3;
        let box_c_start = (c / 3) * 3;
        
        // Check for duplicates in the 3x3 box without a heap allocation.
        // Iterate over the 3 rows and 3 columns of the box.
        for box_row_offset in 0..3 {
            for box_col_offset in 0..3 {
                let current_row = box_r_start + box_row_offset;
                let current_col = box_c_start + box_col_offset;
                if self.board[current_row][current_col] == num {
                    return false;
                }
            }
        }
        // passes all tests
        true
    }

    pub fn is_valid_config(config: &[[u8; 9]; 9]) -> bool {
        // Check rows and columns for duplicates
        for i in 0..9 {
            let mut row_seen = HashSet::with_capacity(9);
            let mut col_seen = HashSet::with_capacity(9);
            for j in 0..9 {
                // Check the current row
                if config[i][j] != 0 {
                    // If the number is already in the set, it's a duplicate.
                    if !row_seen.insert(config[i][j]) {
                        return false;
                    }
                }
                // Check the current column
                if config[j][i] != 0 {
                    if !col_seen.insert(config[j][i]) {
                        return false;
                    }
                }
            }
        }

        // Check 3x3 boxes for duplicates
        for box_row in (0..9).step_by(3) {
            for box_col in (0..9).step_by(3) {
                let mut box_seen = HashSet::with_capacity(9);
                for r in box_row..box_row + 3 {
                    for c in box_col..box_col + 3 {
                        if config[r][c] != 0 {
                            if !box_seen.insert(config[r][c]) {
                                return false;
                            }
                        }
                    }
                }
            }
        }
        // If no duplicates were found, the configuration is valid.
        true
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    // A valid Sudoku configuration for use in tests.
    fn valid_config() -> [[u8; 9]; 9] {
        [
            [0, 0, 6, 0, 4, 0, 0, 9, 7],
            [0, 4, 0, 7, 3, 0, 0, 1, 0],
            [0, 1, 7, 0, 9, 2, 0, 3, 0],
            [6, 0, 0, 0, 7, 0, 0, 8, 0],
            [1, 0, 5, 0, 6, 0, 9, 0, 3],
            [0, 2, 0, 0, 1, 0, 0, 0, 6],
            [0, 5, 0, 9, 8, 0, 1, 6, 0],
            [0, 9, 0, 0, 5, 6, 0, 7, 0],
            [8, 6, 0, 0, 2, 0, 3, 0, 0],
        ]
    }

    // An invalid configuration with a duplicate number in the second row.
    fn invalid_row_config() -> [[u8; 9]; 9] {
        [
            [0, 0, 0, 2, 6, 0, 7, 0, 1],
            [6, 8, 0, 0, 7, 0, 0, 8, 0], // duplicate 8
            [1, 9, 0, 0, 0, 4, 5, 0, 0],
            [8, 2, 0, 1, 0, 0, 0, 4, 0],
            [0, 0, 4, 6, 0, 2, 9, 0, 0],
            [0, 5, 0, 0, 0, 3, 0, 2, 8],
            [0, 0, 9, 3, 0, 0, 0, 7, 4],
            [0, 4, 0, 0, 5, 0, 0, 3, 6],
            [7, 0, 3, 0, 1, 8, 0, 0, 0]
        ]
    }

    // An invalid configuration with a duplicate number in the second column.
    fn invalid_col_config() -> [[u8; 9]; 9] {
        [
            [0, 9, 0, 2, 6, 0, 7, 0, 1],
            [6, 0, 0, 0, 7, 0, 0, 8, 0],
            [1, 9, 0, 0, 0, 4, 5, 0, 0], // duplicate 9
            [8, 2, 0, 1, 0, 0, 0, 4, 0],
            [0, 0, 4, 6, 0, 2, 9, 0, 0],
            [0, 5, 0, 0, 0, 3, 0, 2, 8],
            [0, 0, 9, 3, 0, 0, 0, 7, 4],
            [0, 4, 0, 0, 5, 0, 0, 3, 6],
            [7, 0, 3, 0, 1, 8, 0, 0, 0]
        ]
    }

    // An invalid configuration with a duplicate number in the top-right(1, 3) box.
    fn invalid_box_config() -> [[u8; 9]; 9] {
        [
            [0, 0, 0, 2, 6, 0, 7, 0, 1],
            [6, 0, 0, 0, 7, 0, 0, 8, 0],
            [1, 9, 0, 0, 0, 4, 5, 0, 7], // duplicate 7
            [8, 2, 0, 1, 0, 0, 0, 4, 0],
            [0, 0, 4, 6, 0, 2, 9, 0, 0],
            [0, 5, 0, 0, 0, 3, 0, 2, 8],
            [0, 0, 9, 3, 0, 0, 0, 7, 4],
            [0, 4, 0, 0, 5, 0, 0, 3, 6],
            [7, 0, 3, 0, 1, 8, 0, 0, 0]
        ]
    }

    #[test]
    fn test_init_from_valid_config() {
        // The from() function should succeed with a valid configuration.
        assert!(SudokuBoard::from(valid_config()).is_ok());
    }

    #[test]
    fn test_init_from_invalid_configs() {
        // The from() function should return an error for an invalid configuration.
        assert!(SudokuBoard::from(invalid_row_config()).is_err());
        assert!(SudokuBoard::from(invalid_col_config()).is_err());
        assert!(SudokuBoard::from(invalid_box_config()).is_err());
    }

    #[test]
    fn test_get_value() {
        // Test that get() retrieves the correct value from the board.
        let board = SudokuBoard::from(valid_config()).unwrap();
        assert_eq!(board.get((0, 2)), Some(6)); // Should be 6
        assert_eq!(board.get((0, 0)), Some(0)); // Should be 0 
        assert_eq!(board.get((9, 9)), None);   // Out of bounds
    }

    #[test]
    fn test_set_on_empty_cell() {
        // Test that we can set() a value on an empty cell.
        let mut board = SudokuBoard::from(valid_config()).unwrap();
        assert!(board.set((0, 0), 5).is_ok());
        assert_eq!(board.get((0, 0)), Some(5));
    }

    #[test]
    fn test_set_on_initial_cell_fails() {
        // Test that set() correctly fails when trying to modify a starting number.
        let mut board = SudokuBoard::from(valid_config()).unwrap();
        // Cell (0, 2) was part of the initial config (value is 6).
        assert!(board.set((0, 2), 5).is_err());
    }

    #[test]
    fn test_validate_move_valid() {
        // Test validate_move() for a move that is valid.
        let board = SudokuBoard::from(valid_config()).unwrap();
        // Placing a 2 in cell (0, 0) should be valid.
        assert!(board.validate_move((0, 0), 2));
    }

    #[test]
    fn test_validate_move_invalid_num() {
        // Test validate_move() for a move that is invalid.
        let board = SudokuBoard::from(valid_config()).unwrap();
        // Placing a 12 in cell (0, 0) should be invalid.
        assert!(!board.validate_move((0, 0), 12));
    }

    #[test]
    fn test_validate_move_invalid_row() {
        // Test validate_move() for a move that conflicts with an existing number in the row.
        let board = SudokuBoard::from(valid_config()).unwrap();
        // Placing a 9 in cell (0, 0) should be invalid because 9 is already in row 0.
        assert!(!board.validate_move((0, 0), 9));
    }
    
    #[test]
    fn test_validate_move_invalid_col() {
        // Test validate_move() for a move that conflicts with an existing number in the column.
        let board = SudokuBoard::from(valid_config()).unwrap();
        // Placing a 1 in cell (0, 0) should be invalid because 1 is already in column 0.
        assert!(!board.validate_move((0, 0), 1));
    }

    #[test]
    fn test_validate_move_invalid_box() {
        // Test validate_move() for a move that conflicts with an existing number in the 3x3 box.
        let board = SudokuBoard::from(valid_config()).unwrap();
        // Placing a 7 in cell (0, 0) should be invalid because 7 is already in the top-left box.
        assert!(!board.validate_move((0, 0), 7));
    }

    #[test]
    fn test_validate_move_overwrite_placed_fails() {
        // Test that validate_move() returns false when trying to overwrite a starting number.
        let board = SudokuBoard::from(valid_config()).unwrap();
        // Trying to place a 5 on cell (0, 2), which already contains a 6.
        assert!(!board.validate_move((0, 2), 5));
    }

    #[test]
    fn test_validate_move_overwrite_starting_fails() {
        // Test that validate_move() returns false when trying to overwrite a placed number.
        let mut board = SudokuBoard::from(valid_config()).unwrap();
        // Placing a 3 on an empty square, and trying to overwrite it with a 5.
        board.set((0, 1), 3);
        assert!(!board.validate_move((0, 1), 5));
    }

    #[test]
    fn test_validate_move_clear_cell_is_valid() {
        // Test that validate_move() returns true when clearing a cell (placing a 0).
        let mut board = SudokuBoard::from(valid_config()).unwrap();
        // First, place a valid number.
        board.set((0,0), 2).unwrap();
        // Now, try to clear it.
        assert!(board.validate_move((0, 0), 0));
    }
}
