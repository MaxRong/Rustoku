mod sudoku_board;
mod sudoku_solver;

use sudoku_board::SudokuBoard;
use sudoku_solver::BacktrackingSolver;

fn main() {
    let config = [
        [0, 0, 6, 0, 4, 0, 0, 9, 7],
        [0, 4, 0, 7, 3, 0, 0, 1, 0],
        [0, 1, 7, 0, 9, 2, 0, 3, 0],
        [6, 0, 0, 0, 7, 0, 0, 8, 0],
        [1, 0, 5, 0, 6, 0, 9, 0, 3],
        [0, 2, 0, 0, 1, 0, 0, 0, 6],
        [0, 5, 0, 9, 8, 0, 1, 6, 0],
        [0, 9, 0, 0, 5, 6, 0, 7, 0],
        [8, 6, 0, 0, 2, 0, 3, 0, 0],
    ];
    
    let mut board: SudokuBoard = SudokuBoard::from(config).expect("Build failed"); // will panic if config is invalid.
    let mut solved_board = BacktrackingSolver::run(&board).unwrap();
    board.print();
    println!("{}", "-".repeat(31));
    solved_board.print();

}