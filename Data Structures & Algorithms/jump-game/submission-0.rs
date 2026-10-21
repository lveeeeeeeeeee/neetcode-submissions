impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }
        let mut ans: bool = false;
        // guaranteed to be more than 1 because we returned early in case of len == 1
        let mut i: usize = nums.len() - 2;
        let mut ask: usize = nums.len() - 1;
        while true {
            if i + nums[i] as usize >= ask {
                ask = i;
                ans = true;
            } else {
                ans = false;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        ans
    }
}
