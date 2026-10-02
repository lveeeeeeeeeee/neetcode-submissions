use std::iter::zip;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map: [u32; 26] = [0; 26];
        for (sc, tc) in s.chars().zip(t.chars()) {
            map[sc as usize - 'a' as usize] += 1;
            map[tc as usize - 'a' as usize] -= 1;
        }
        for cnt in map {
            if cnt != 0 {
                return false;
            }
        }
        true
    }
}
