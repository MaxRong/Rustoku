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
    pub fn build(config: [[u8; 9]; 9]) -> Result<(Self), &'static str> {
        if !Self::is_valid_config(&config) {
            return Err("Error: Invalid config used in SudokuBoard::build().");
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
        // Convert cell coordinate tuple to usize row & column coords for indexing
        let (r, c) = (cell.0 as usize, cell.1 as usize);

        // Check that cell is within bounds, and move type is valid
        match self.get(cell) {
            None => { return false }, // If cell is out of bounds, it's invalid
            Some(value) => {
                // A move is invalid if you try to place a non-zero number
                // onto a cell that already contains a non-zero number
                if value !=0 && num != 0 { return false } 
            },
        }

        // If move num is 0, currently always return true.
        if num == 0 { return true }

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

    // An invalid configuration with a duplicate number in the second col.
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
    fn test_build_valid_config() {
        // The `build` function should succeed with a valid configuration.
        assert!(SudokuBoard::build(valid_config()).is_ok());
    }

    #[test]
    fn test_build_invalid_configs() {
        // The `build` function should return an error for an invalid configuration.
        assert!(SudokuBoard::build(invalid_row_config()).is_err());
        assert!(SudokuBoard::build(invalid_col_config()).is_err());
        assert!(SudokuBoard::build(invalid_box_config()).is_err());
    }

    #[test]
    fn test_get_value() {
        // Test that get() retrieves the correct value from the board.
        let board = SudokuBoard::build(valid_config()).unwrap();
        assert_eq!(board.get((0, 2)), Some(6)); // Should be 6
        assert_eq!(board.get((0, 0)), Some(0)); // Should be 0 
        assert_eq!(board.get((9, 9)), None);   // Out of bounds
    }

    #[test]
    fn test_set_on_empty_cell() {
        // Test that we can set() a value on an empty cell.
        let mut board = SudokuBoard::build(valid_config()).unwrap();
        assert!(board.set((0, 0), 5).is_ok());
        assert_eq!(board.get((0, 0)), Some(5));
    }
}
