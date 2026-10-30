impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        let mut dp: Vec<i32> = vec![-1; 2 + amount as usize];
        coins.sort_unstable();
        for i in 0..dp.len() {
            if i == 0 {
                dp[0] = 0;
                continue;
            }
            let mut min_path: Option<i32> = None;
            let mut coin: usize = 0;
            while coin < coins.len() && coins[coin] as usize <= i {
                let coin_ind = coins[coin] as usize;
                if dp[i - coin_ind] != -1 || i - coin_ind == 0 {
                    min_path = if let Some(val) = min_path {
                        // println!("replaced val: {}", dbg);
                        Some(val.min(dp[i - coin_ind] + 1))
                    }
                    else {
                        Some(dp[i - coin_ind] + 1)
                    };
                }
                coin += 1;
            }
            if let Some(val) = min_path {
                dp[i] = val;
            }
        }
        dp[amount as usize]
    }
}
