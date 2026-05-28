use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // validate rows
        for i in 0..9 {
            let mut set = HashSet::new();
            for j in 0..9 {
                if board[i][j] != '.' && set.contains(&board[i][j]) {
                    return false;
                }
                set.insert(board[i][j]);
            }
        }

        // validate columns
        for i in 0..9 {
            let mut set = HashSet::new();
            for j in 0..9 {
                let val = board[j][i];
                if val != '.' && set.contains(&val) {
                    return false;
                }
                set.insert(val);
            }
        }

        // validate subs
        for i in 0..3 {
            for j in 0..3 {
                let mut set = HashSet::new();
                for k in 0..3 {
                    for l in 0..3 {
                        let val = board[i * 3 + k][j * 3 + l];
                        if val != '.' && set.contains(&val) {
                            return false;
                        }
                        set.insert(val);
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        let board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        assert!(Solution::is_valid_sudoku(board));
    }
}
