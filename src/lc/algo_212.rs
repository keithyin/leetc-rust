use std::collections::btree_map::BTreeMap;
use std::collections::HashSet;

#[derive(Debug)]
struct TrieNode {
    word: String,
    next_edges: BTreeMap<char, TrieNode>,
}


impl TrieNode {
    pub fn build_trie_tree(words: &Vec<String>) -> Self {
        let mut trie_tree = TrieNode{word: "".to_string(), next_edges:BTreeMap::new()};
        for word in words.iter() {
            let mut cursor = &mut trie_tree;
            for c in word.chars() {
                if !cursor.next_edges.contains_key(&c) {
                    cursor.next_edges.insert(c, TrieNode::new("".to_string(), '\0'));
                }
                cursor = if let Some(inner) = cursor.next_edges.get_mut(&c) {
                    inner
                } else {
                    panic!("")
                };
            }
            cursor.word = word.clone();

        }
        trie_tree
    }

    pub fn new(word: String, c: char) -> Self {
        if c == '\0' {
            return TrieNode{word, next_edges: BTreeMap::new()}
        }

        let mut edges = BTreeMap::new();
        edges.insert(c, TrieNode::new("".to_string(), '\0'));
        TrieNode{word, next_edges: edges}
    }
}


pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
    let mut trie_tree = TrieNode::build_trie_tree(&words);
    let mut result = HashSet::new();
    for cur_row in 0..board.len() {
        for cur_col in 0..board[0].len() {
            dfs(&mut board, &trie_tree, cur_row, cur_col, &mut result);
        }
    }
    result.into_iter().collect()

}

fn dfs(board: &mut Vec<Vec<char>>, mut cur_trie_node: & TrieNode, cur_row: usize, cur_col: usize, result: &mut HashSet<String>) {
    let tot_row = board.len() as i32;
    let tot_col = board[0].len() as i32;

    let cur_char = board[cur_row][cur_col];

    if cur_trie_node.next_edges.contains_key(&cur_char) {
        cur_trie_node = if let Some(inner) = cur_trie_node.next_edges.get(&cur_char) {
            inner
        } else {
            panic!("")
        }
    } else {
        return;
    }

    if !cur_trie_node.word.eq("") {
        result.insert(cur_trie_node.word.clone());
    }


    board[cur_row][cur_col] = '#';

    for (x, y) in [(cur_row as i32 - 1, cur_col as i32), (cur_row as i32 + 1, cur_col as i32),
        (cur_row as i32, cur_col as i32 - 1), (cur_row as i32, cur_col as i32 +1)] {

        if x >= 0 && x < tot_row && y >= 0 && y < tot_col && board[x as usize][y as usize] != '#' {
            dfs(board, cur_trie_node, x as usize, y as usize, result);
        }
    }

    board[cur_row][cur_col] = cur_char;
}

#[cfg(test)]
mod test {
    use crate::lc::algo_212::{find_words, TrieNode};

    #[test]
    fn test_trie_tree() {
        let words = vec!["hello".to_string(), "world".to_string(), "hell".to_string()];
        let trie_tree  = TrieNode::build_trie_tree(&words);
        let board = vec![vec!['h', 'e'], vec!['b', 'c']];
        println!("{:?}", find_words(board, words));
    }
}