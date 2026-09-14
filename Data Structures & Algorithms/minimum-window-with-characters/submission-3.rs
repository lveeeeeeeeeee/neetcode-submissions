impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // unfortunately same implication that 
        // you have to keep the counts of the symbols
        // this is a "check if s contains permutation of t"
        // but also "find the length and the index of 
        // the shortest substring of s that contains permutation of t"
        // solution: find the substrings with permutation,
        // for each one: while substring satisfies the condition,
        // shorten it until it doesn't
        // then look for the next substring
        // store the min length and the index from which the substring starts
        // if substring.len() == t.len() you can also return that substring
        // letter case unfortunately matters
        let res: String = String::from("");
        if s.len() < t.len() {
            return res;
        }
        let sb = s.as_bytes();
        let tb = t.as_bytes();
        let map = |x: u8| -> u8  { if x > 25 { x - 6 } else { x } };
        let mut count_mask: [i32; 52] = [0; 52];
        let mut mask: u64 = 0;
        for i in 0..tb.len() {
            let ch_rep = map(tb[i] - 'A' as u8);
            count_mask[ch_rep as usize] += 1;
            if mask & (1 << ch_rep as u64) == 0 {
                mask ^= 1 << ch_rep as u64;
            }
        }
        let mut curr_mask: [i32; 52] = [0; 52];
        let mut l: usize = 0;
        let mut r: usize = 0;
        let mut min_len: usize = s.len();
        let mut min_ind: usize = 0;
        let mut satisfied: bool = false;
        let mut contains_permut: bool = false;
        while r < s.len() {
            if !satisfied {
                let ch_rep_r = map(sb[r] - 'A' as u8);
                if mask & (1 << ch_rep_r as u64) != 0 {
                    curr_mask[ch_rep_r as usize] += 1;
                }
                r += 1;
            }
            loop {
                let mut i: usize = 0;
                satisfied = true;
                while i < count_mask.len() && satisfied {
                    if mask & (1 << i as u64) != 0 {
                        satisfied &= curr_mask[i] >= count_mask[i];
                    }
                    i += 1;
                }
                if satisfied && l < r {
                    contains_permut = true;
                    let ch_rep_l = map(sb[l] - 'A' as u8);
                    if min_len > r - l {
                        min_ind = l;
                        min_len = min_len.min(r - min_ind);
                    }
                    if min_len == t.len() { 
                        return String::from(&s[min_ind..min_ind+t.len()]); 
                    }
                    curr_mask[ch_rep_l as usize] -= 1;
                    l += 1;
                } else {
                    break;
                }
            }
        }
        if contains_permut { 
            return String::from(&s[min_ind..min_ind + min_len]);
        }
        String::default()
    }
}