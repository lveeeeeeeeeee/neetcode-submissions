impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut max = 0;
        let mut dp: Vec<(i32, usize)> = vec![(0,0usize); nums.len()];
        for i in 0..nums.len() {
            if i == 0 {
                dp[i] = (nums[i], 0);
                max = nums[i];
            } else if i == 1 {
                if nums[i] > dp[i-1].0 {
                    dp[i] = (nums[i], i);
                } else {
                    dp[i] = dp[i-1];
                }
                max = max.max(dp[i].0);
            } else { 
                let (rob, h) = dp[i-2];
                let (sob, hs) = dp[i-1];
                if i - hs <= 1 {
                    if rob + nums[i] > sob {
                        dp[i] = (rob + nums[i], i);
                    } else {
                        dp[i] = (sob, hs);
                    }
                } else {
                    if rob >= sob {
                        dp[i] = (rob+nums[i], i);
                    } else {
                        dp[i] = (sob+nums[i], i);
                    }
                }
                max = max.max(dp[i].0);
            }
            println!("{:?}", dp[i]);
        }
        max
    }
}
