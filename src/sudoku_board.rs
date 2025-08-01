// Sudoku Board Module
use std::collections::HashSet;

#[derive(Clone, Copy)]
pub struct SudokuBoard {
    board: [[u8; 9]; 9],
}

impl SudokuBoard {
    // Class Constructor
    // Assume config always exists for now.
    pub fn build(config: [[u8; 9]; 9]) -> Result<(Self), &'static str> {
        if !Self::is_valid_config(&config) {
            return Err("Error: Invalid config used in SudokuBoard::build().");
        }
        Ok(SudokuBoard { board: config })
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
        // Modify cell on board
        if let Some(row) = self.board.get_mut(cell.0 as usize) {
            if let Some(elem) = row.get_mut(cell.1 as usize) {
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

        // Check that cell is within bounds, and is empty(0)
        match self.get(cell) {
            Some(0) => {} // If cell is empty, do nothing and continue.
            _ => return false, // If None or Some(non-zero), return false.
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



// TODO: Unit tests