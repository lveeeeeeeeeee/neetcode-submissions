impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; cost.len() + 1];
        if cost.len() == 2 {
            return cost[0].min(cost[1]);
        }
        for i in 2..dp.len() {
            let one = dp[i-1] + cost[i-1];
            let two = dp[i-2] + cost[i-2];
            dp[i] = one.min(two);
            // println!("{}", &dp[i]);
        }
        dp[dp.len() - 1]
    }
}
