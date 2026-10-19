impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut steal_max: i32 = 0;
        let mut wait_max: i32 = 0;
        for (i, &num) in nums.iter().enumerate() {
            if i == 0 {
                wait_max = num;
            } else {
                steal_max = wait_max.max(steal_max + num);
                (steal_max, wait_max) = (wait_max, steal_max);
            }
        }
        steal_max.max(wait_max)
    }
}
