use std::collections::HashSet;

pub fn valid_sudoku(board:Vec<Vec<char>>) -> bool {
    let mut rows:Vec<HashSet<i32>> = vec![HashSet::new(); 9];
    let mut cols:Vec<HashSet<i32>> = vec![HashSet::new(); 9];
    let mut boxs:Vec<HashSet<i32>> = vec![HashSet::new(); 9];

    for row in 0..9usize {
        for col in 0..9usize {
            let c:char = board[row][col];
            if c == '.' {
                continue;
            }
            let n:i32 = c.to_digit(10).unwrap() as i32;
            let box_idx:usize = (row/3)*3 + (col/3);
            if rows.get(row).unwrap().contains(&n) ||
                cols.get(col).unwrap().contains(&n) ||
                boxs.get(box_idx).unwrap().contains(&n) {
                return false;
            }
            rows.get_mut(row).unwrap().insert(n);
            cols.get_mut(col).unwrap().insert(n);
            boxs.get_mut(box_idx).unwrap().insert(n);
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        let board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert_eq!(valid_sudoku(board), true);
    }

    #[test]
    fn test_invalid() {
        let board = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert_eq!(valid_sudoku(board), false);
    }
}
