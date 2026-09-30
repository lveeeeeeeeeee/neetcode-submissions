impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        // just do the O(n) 
        // meaning keeping the counts of symbols and comparing to the reference
        // it is 26 iterations but like i mean its O(n) right
        if s1.len() > s2.len() { return false; }
        let sb = s1.as_bytes();
        let tb = s2.as_bytes();
        let mut count_mask: [i32; 26] = [0; 26];
        let mut curr_mask: [i32; 26] = [0; 26];
        for num in sb {
            let ch = (num - 'a' as u8) as usize;
            count_mask[ch] += 1;
        }
        let mut r = 0;
        let mut l = 0;
        while r < tb.len() {
            let ch_r = (tb[r] - 'a' as u8) as usize;
            curr_mask[ch_r] += 1;
            r += 1;
            if r - l == sb.len() {
                let mut result = true;
                for i in 0..count_mask.len() {
                    if count_mask[i] != curr_mask[i] {
                        result = false;
                        break;
                    }
                }
                if result { return result; }
                let ch_l = (tb[l] - 'a' as u8) as usize;
                curr_mask[ch_l] -= 1;
                l += 1;
            }
        }
        false
    }
}