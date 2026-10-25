use std::collections::HashSet;

impl Solution {
    pub fn same(ch: u8, other: char) -> bool {
        return ch == other as u8;
    }

    pub fn is_taken(coords: (usize, usize), map: &Vec<Option<(usize, usize)>>) -> bool {
        let mut i = 0;
        while i < map.len() && let Some(elem) = map[i] {
            if coords.0 == elem.0 && coords.1 == elem.1 {
                return true;
            }
            i += 1;
        }
        false
    }

    // probably dfs
    // go and collect the word
    // check last w.len() elements to see if we met a word
    // if not met a word, backtrack
    // if we backtracked all the way back and havent found a word, there's no word then
    // n * m * 4^(word.len) upper bound due to us choosing where to go for each element
    // at most 4 choices, c1,4 = 4!/1!3! = 4
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let wordb: &[u8] = word.as_bytes();
        let mut map: Vec<Option<(usize, usize)>> = vec![None; wordb.len()];

        fn dfsd(word: &[u8], board: &Vec<Vec<char>>, row: usize, col: usize, i: usize, map: &mut Vec<Option<(usize, usize)>>) -> bool {
            if !Solution::same(word[i], board[row][col]) {
                map[i] = None;
                return false;
            }
            if i == word.len() - 1 {
                map[i] = None;
                return Solution::same(word[i], board[row][col]);
            }
            map[i] = Some((row, col));
            let u = if row > 0 && !Solution::is_taken((row-1, col), map) {
                dfsd(word, board, row-1, col, i+1, map) 
            } else { false };
            let l = if col > 0 && !Solution::is_taken((row, col-1), map) { 
                dfsd(word, board, row, col-1, i+1, map) 
            } else { false };
            let d = if row < board.len()-1 && !Solution::is_taken((row+1, col), map) { 
                dfsd(word, board, row+1, col, i+1, map) 
            } else { false };
            let r = if col < board[0].len()-1 && !Solution::is_taken((row, col+1), map) { 
                dfsd(word, board, row, col+1, i+1, map) 
            } else { false };
            map[i] = None;
            u || l || d || r
        }

        for row in 0..board.len() {
            for col in 0..board[row].len() {
                let res = dfsd(&wordb, &board, row, col, 0, &mut map);
                if res {return res}
            }
        }
        false
    }
}
