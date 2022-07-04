struct Solution;

use std::collections::HashSet;

impl Solution {
    fn validate(board: &Vec<Vec<char>>, positions: [(usize, usize); 9]) -> bool {
        let mut hash_set = HashSet::new();
        for pos in positions {
            if board[pos.0][pos.1].is_numeric() {
                if hash_set.get(&board[pos.0][pos.1]).is_some() {
                    return false;
                }
                hash_set.insert(board[pos.0][pos.1]);
            }
        }

        true
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let top_left: [(usize, usize); 9] = [
            (0, 0),
            (0, 3),
            (0, 6),
            (3, 0),
            (3, 3),
            (3, 6),
            (6, 0),
            (6, 3),
            (6, 6),
        ];
        for index in 0..9 {
            let v_position = [
                (index, 0),
                (index, 1),
                (index, 2),
                (index, 3),
                (index, 4),
                (index, 5),
                (index, 6),
                (index, 7),
                (index, 8),
            ];

            let h_positions = [
                (0, index),
                (1, index),
                (2, index),
                (3, index),
                (4, index),
                (5, index),
                (6, index),
                (7, index),
                (8, index),
            ];

            let b_positions = [
                top_left[index],
                (top_left[index].0, top_left[index].1 + 1),
                (top_left[index].0, top_left[index].1 + 2),
                (top_left[index].0 + 1, top_left[index].1),
                (top_left[index].0 + 1, top_left[index].1 + 1),
                (top_left[index].0 + 1, top_left[index].1 + 2),
                (top_left[index].0 + 2, top_left[index].1),
                (top_left[index].0 + 2, top_left[index].1 + 1),
                (top_left[index].0 + 2, top_left[index].1 + 2),
            ];

            if !(Self::validate(&board, v_position)
                && Self::validate(&board, h_positions)
                && Self::validate(&board, b_positions))
            {
                return false;
            }
        }

        true
    }
}

pub fn run() {
    let board = [
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'].to_vec(),
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'].to_vec(),
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'].to_vec(),
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'].to_vec(),
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'].to_vec(),
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'].to_vec(),
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'].to_vec(),
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'].to_vec(),
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'].to_vec(),
    ]
    .to_vec();
    let result = Solution::is_valid_sudoku(board);
    println!("{}", result);
}
