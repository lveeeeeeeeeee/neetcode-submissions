impl Solution {
    pub fn hamming_weight(mut n: u32) -> i32 {
        let mut cnt = 0;
        while n != 0 {
            cnt += 1;
            n = n & (n - 1);
            if n != 0 {
                n = n >> n.trailing_zeros();
            }
        }
        cnt
    }
}
