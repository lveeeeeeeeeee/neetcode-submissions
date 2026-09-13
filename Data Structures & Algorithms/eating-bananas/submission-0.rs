impl Solution {
    pub fn check(piles: &Vec<i32>, h: i32, rate: i32) -> bool {
        let mut h = h;
        for elem in piles {
            if elem % rate == 0 {
                h -= elem / rate;
            } else {
                h -= (rate - (elem % rate) + elem) / rate;   
            }
        }
        h >= 0
    }

    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut mx = 1_000_000_000;
        let mut mn = 1;
        while mn <= mx {
            let m = mn + (mx - mn) / 2;
            if Self::check(&piles, h, m) {
                mx = m - 1;
            }
            else {
                mn = m + 1;
            }
        }
        mn
    }
}
