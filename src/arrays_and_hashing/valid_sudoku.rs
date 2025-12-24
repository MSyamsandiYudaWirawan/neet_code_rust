use std::collections::HashSet;

pub fn valid_sudoku(board:Vec<Vec<char>>) -> bool {
    let mut rows:Vec<HashSet<u32>> = Vec::new();
    let mut cols:Vec<HashSet<u32>> = Vec::new();
    let mut boxes:Vec<HashSet<u32>> = Vec::new();
    
    for _ in 0..9 {
        rows.push(HashSet::new());
        cols.push(HashSet::new());
        boxes.push(HashSet::new());
    }
    for r in 0..9 {
        for c in 0..9 {
            let car:char = board[r][c];
            if car == '.' {
                continue;
            }
            let num:u32 = car.to_digit(10).unwrap();
            let box_num:usize = (r/3)*3 + (c/3);
            if rows[r].contains(&num) || cols[c].contains(&num) || boxes[box_num].contains(&num){
                return false;
            }
            rows[r].insert(num);
            cols[c].insert(num);
            boxes[box_num].insert(num);
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
