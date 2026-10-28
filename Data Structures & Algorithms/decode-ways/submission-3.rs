impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let s = s.as_bytes();
        let (mut dp2, mut dp1) = (0i32, 1i32);
        let mut dp = 0i32;
        for i in (0..s.len()).rev() {
            let elem = s[i] - '0' as u8;
            let prev = if i + 1 < s.len() { s[i+1] - '0' as u8 } else { 0 };
            if elem == 0 {
                dp = 0;
            }
            else {
                dp = dp1;
                if i + 1 < s.len() && (elem == 1 || (elem == 2 && prev < 7)) {
                    dp += dp2;
                }
            }
            dp2 = dp1;
            dp1 = dp;
            dp = 0;
        }
        dp1
    }
}
