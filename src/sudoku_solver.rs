use crate::sudoku_board::SudokuBoard;

pub struct BacktrackingSolver;

impl BacktrackingSolver {
    /// The recursive helper function that implements the backtracking logic.
    pub fn run(init_board: &SudokuBoard) -> Option<SudokuBoard> {
        // returns false if unsolvable.
        let mut board = *init_board;
        if Self::recursive_solve(&mut board) {
            // board is guaranteed to be valid if solved by properties of SudokuBoard
            return Some(board);
        }
        None
    }

    fn recursive_solve(board: &mut SudokuBoard) -> bool {
        if let Some(empty_cell) = Self::find_first_empty_cell(board) {
            for num in 1..=9 {
                if board.is_placement_valid(empty_cell, num) {
                    board.internal_place(empty_cell, num);
                    if Self::recursive_solve(board) {
                        return true;
                    }
                    board.internal_place(empty_cell, 0); 
                }
            }
            false // No number worked, need to backtrack
        } else {
            true // No empty cells, board is solved
        }
    }

    fn find_first_empty_cell(board: &SudokuBoard) -> Option<(u8, u8)> {
        for r in 0..9 {
            for c in 0..9 {
                if board.get((r, c)) == Some(0) {
                    return Some((r, c));
                }
            }
        }
        // No empty cells were found.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A solvable Sudoku configuration for use in tests.
    fn solvable_config() -> [[u8; 9]; 9] {
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

    // An unsolvable but valid configuration.
    fn unsolvable_config() -> [[u8; 9]; 9] {
        [
            [1, 2, 3, 4, 5, 6, 7, 8, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 9], // The 9 here makes it impossible to place a 9 in the first row
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ]
    }

    #[test]
    fn test_backtrack_solver_success() {
        let board = SudokuBoard::from(solvable_config()).unwrap();
        assert!(BacktrackingSolver::run(&board).is_some());
    }

    #[test]
    fn test_backtrack_solver_fail() {
        let board: SudokuBoard = SudokuBoard::from(unsolvable_config()).unwrap();
        assert!(BacktrackingSolver::run(&board).is_none());
    }
}