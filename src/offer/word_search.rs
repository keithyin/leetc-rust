use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::process::id;
use std::rc::Rc;

pub fn search(board: &Vec<Vec<char>>, word: &Vec<char>, checked: Rc<RefCell<Vec<Vec<bool>>>>, row: usize, col: usize, idx: usize) -> bool {
    if idx == word.len() {
        return true;
    }
    let mut found = false;
    let tot_row = board.len();
    let tot_col = board[0].len();

    // down
    if row < (tot_row-1) && !checked.as_ref().borrow_mut()[row+1][col] && board[row+1][col]== word[idx]{
        checked.as_ref().borrow_mut()[row+1][col] = true;
        found = search(board, word, Rc::clone(&checked), row+1, col, idx+1);
        checked.as_ref().borrow_mut()[row+1][col] = false;
    }
    if found {
        return found;
    }
    // up
    if row > 0 && !checked.as_ref().borrow_mut()[row-1][col] && board[row-1][col] == word[idx]{
        checked.as_ref().borrow_mut()[row-1][col] = true;
        found = search(board, word, Rc::clone(&checked), row-1, col, idx+1);
        checked.as_ref().borrow_mut()[row-1][col] = false;
    }
    if found {
        return found;
    }

    // right
    if col < (tot_col-1) && !checked.as_ref().borrow_mut()[row][col+1] && board[row][col+1] == word[idx]{
        checked.as_ref().borrow_mut()[row][col+1] = true;
        found = search(board, word, Rc::clone(&checked), row, col+1, idx+1);
        checked.as_ref().borrow_mut()[row][col+1] = false;
    }
    if found {
        return found;
    }

    // left
    if col > 0 && !checked.as_ref().borrow_mut()[row][col-1] && board[row][col-1] == word[idx]{
        checked.as_ref().borrow_mut()[row][col-1] = true;
        found = search(board, word, Rc::clone(&checked), row, col-1, idx+1);
        checked.as_ref().borrow_mut()[row][col-1] = false;
    }
    if found {
        return found;
    }

    false
}

pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    let num_row = board.len();
    let num_col = board[0].len();
    let word: Vec<char> = word.chars().collect();

    let checked = Rc::new(RefCell::new(vec![vec![false; num_col]; num_row]));
    let mut found = false;
    for i in 0..num_row {
        for j in 0..num_col {
            checked.as_ref().borrow_mut()[i][j] = true;
            if board[i][j] == word[0] {
                found = search(&board, &word, Rc::clone(&checked), i, j, 1);
                if found {
                    return found;
                }
            }
            checked.as_ref().borrow_mut()[i][j] = false;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::offer::word_search::exist;

    #[test]
    pub fn vec_g() {
        println!("{:?}", vec![0; 10]);
    }

    #[test]
    pub fn test_exist() {
        let board = vec![vec!['A', 'B'], vec!['C', 'D']];
        let word = "CAD".to_string();
        println!("{:?}", exist(board, word));

    }
}