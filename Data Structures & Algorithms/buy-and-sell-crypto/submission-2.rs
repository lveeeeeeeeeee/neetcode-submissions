impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut l: usize = 0;
        let mut r: usize = 0;
        loop {
            if r >= prices.len() {
                break;
            }
            if prices[l] >= prices[r] {
                l = r;
            }
            else {
                res = res.max(prices[r]-prices[l]);
                println!{"{l:?}: {}; {r:?}: {}; {res:?}", prices[l], prices[r]};
            }
            r += 1;
        } 
        res
    }
}
