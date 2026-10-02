use std::iter::zip;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let deduct = 'a' as usize;
        let mut map: [u32; 26] = [0; 26];
        for (sc, tc) in s.chars().zip(t.chars()) {
            map[sc as usize - deduct] += 1;
            map[tc as usize - deduct] -= 1;
        }
        map.iter().all(|&c| c == 0)
    }
}
