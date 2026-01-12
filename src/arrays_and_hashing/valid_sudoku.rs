use std::collections::HashSet;

pub fn valid_sudoku(board:Vec<Vec<char>>) -> bool {
    let mut rows:Vec<HashSet<i32>> = vec![HashSet::new(); 9];
    let mut cols:Vec<HashSet<i32>> = vec![HashSet::new(); 9];
    let mut boxs:Vec<HashSet<i32>> = vec![HashSet::new(); 9];

    for row in 0..9 as usize {
        for col in 0..9 as usize{
            let c:char = board[row][col];
            if c == '.'{
                continue;
            }
            let num:i32 = c.to_digit(10).unwrap() as i32;
            let box_index:usize = (row/3)*3 + (col/3);
            if rows.get(row).unwrap().contains(&num) ||
            cols.get(col).unwrap().contains(&num) ||
            boxs.get(box_index).unwrap().contains(&num)
            {
                return false;
            }
            rows.get_mut(row).unwrap().insert(num);
            cols.get_mut(col).unwrap().insert(num);
            boxs.get_mut(box_index).unwrap().insert(num);

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
