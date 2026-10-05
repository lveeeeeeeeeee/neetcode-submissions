impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (small, big) = if nums1.len() <= nums2.len() {(&nums1, &nums2)} else {(&nums2, &nums1)};
        if small.len() == big.len() && big.len() == 0 {
            return 0f64;
        }
        if small.len() == big.len() && big.len() == 1 {
            return (small[0] + big[0]) as f64 / 2f64;
        }
        if small.len() == 0 {
            let n = big.len() / 2;
            if big.len() & 1 == 0 {
                return (big[n] + big[n-1]) as f64 / 2f64;
            } else {
                return big[n] as f64;
            }
        }
        let mut l = 0;
        let mut r = small.len();
        let total = small.len() + big.len();
        let half = (total + 1) / 2;
        loop {
            let sp = (l + r) / 2;
            let bp = half - sp;
            let small_l = if sp > 0 { small[sp-1] as f64 } else { f64::NEG_INFINITY };
            let small_r = if sp < small.len() { small[sp] as f64 } else { f64::INFINITY };
            let big_l = if bp > 0 { big[bp-1] as f64 } else { f64::NEG_INFINITY };
            let big_r = if bp < big.len() { big[bp] as f64 } else { f64::INFINITY };
            if small_l <= big_r && big_l <= small_r {
                if total & 1 == 0 {
                    return (small_l.max(big_l) + small_r.min(big_r)) / 2f64
                } else {
                    return small_l.max(big_l);
                }
            } else {
                if small_l > big_r {
                    r = sp - 1;
                } else {
                    l = sp + 1;
                }
            }
        }
    }
}
