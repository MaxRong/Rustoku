mod sudoku_board;

use sudoku_board::SudokuBoard;

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
    
    let mut board: SudokuBoard = SudokuBoard::build(config).expect("Build failed"); // will panic if config is invalid.

    board.print();

}

/* 
Saved 'faulty' config for tests.
[
[0, 0, 0, 2, 6, 0, 7, 0, 1],
[6, 8, 0, 0, 7, 0, 0, 8, 0],
[1, 9, 0, 0, 0, 4, 5, 0, 0],
[8, 2, 0, 1, 0, 0, 0, 4, 0],
[0, 0, 4, 6, 0, 2, 9, 0, 0],
[0, 5, 0, 0, 0, 3, 0, 2, 8],
[0, 0, 9, 3, 0, 0, 0, 7, 4],
[0, 4, 0, 0, 5, 0, 0, 3, 6],
[7, 0, 3, 0, 1, 8, 0, 0, 0]
];  */