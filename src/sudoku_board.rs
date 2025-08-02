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
    
    // Gets the value of a cell at the given coordinates.
    pub fn get(&self, cell: (u8, u8)) -> Option<u8> {
        // Validates that the cell is on the board and returns value if it is
        self.board
            .get(cell.0 as usize)?
            .get(cell.1 as usize)
            .copied()
    }

    pub fn try_place(&mut self, cell: (u8, u8), num: u8) -> Result<(), &'static str> {
        let (r, c) = (cell.0 as usize, cell.1 as usize);

        // Cell must be valid.
        if r >= 9 || c >= 9 {
            return Err("Error: Cell is out of bounds.");
        }

        // Modifying starting number is invalid.
        if self.initial_mask[r][c] {
            return Err("Error: Cannot modify a starting number.");
        }

        // Clearing a cell is always valid.
        if num == 0 {
            self.board[r][c] = 0;
            return Ok(());
        }

        // Writing over a filled cell is invalid.
        if self.board[r][c] != 0 {
            return Err("Error: Cannot overwrite filled cell. Try clearing first.")
        }

        // Number must be valid (1-9).
        if !(1..=9).contains(&num) {
            return Err("Error: Number must be between 1 and 9.");
        }

        // Check for Sudoku rule conflicts.
        if !self.is_placement_valid(cell, num) {
            return Err("Error: Move conflicts with another number.");
        }

        // If all checks pass, make the move.
        self.board[r][c] = num;
        Ok(())
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

    // Private helper to check if placing a number would violate Sudoku rules.
    // Assumes cell and num are valid.
    fn is_placement_valid(&self, cell: (u8, u8), num: u8) -> bool {
        let (r, c) = (cell.0 as usize, cell.1 as usize);

        // Check for duplicates in the same row and column, ignoring the cell itself.
        for i in 0..9 {
            if self.board[r][i] == num && i != c { return false; }
            if self.board[i][c] == num && i != r { return false; }
        }

        // Check for duplicates in the 3x3 box, ignoring the cell itself.
        let box_start_row = r - r % 3;
        let box_start_col = c - c % 3;
        for i in 0..3 {
            for j in 0..3 {
                let current_row = box_start_row + i;
                let current_col = box_start_col + j;
                if self.board[current_row][current_col] == num && (current_row != r || current_col != c) {
                    return false;
                }
            }
        }
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
    fn test_try_place_scenarios() {
        let mut board = SudokuBoard::from(valid_config()).unwrap();

        // --- Success Cases ---
        // Placing a valid number on an empty cell.
        assert!(board.try_place((0, 0), 2).is_ok());
        assert_eq!(board.get((0, 0)), Some(2));
        
        // Clearing a placed number.
        assert!(board.try_place((0, 0), 0).is_ok());
        assert_eq!(board.get((0, 0)), Some(0));

        // --- Failure Cases ---
        // Overwriting a placed number with non-0 number
        board.try_place((0, 0), 3);
        assert!(!board.try_place((0, 0), 4).is_ok());

        board.try_place((0, 0), 0); // clear placed number from previous test.

        // Trying to place on an initial number.
        assert!(board.try_place((0, 2), 5).is_err());

        // Trying to place out of bounds.
        assert!(board.try_place((9, 9), 5).is_err());

        // Trying to place an invalid number.
        assert!(board.try_place((0, 0), 10).is_err());

        // Trying to place a number that conflicts with a row.
        assert!(board.try_place((0, 0), 9).is_err());

        // Try to place a number that conflicts with a column.
        assert!(board.try_place((0, 0), 1).is_err());

        // Try to place a number that conflicts with a box.
        assert!(board.try_place((0, 0), 4).is_err());
    }
}
