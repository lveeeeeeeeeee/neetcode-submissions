impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut l = 0usize;
        let mut r = nums.len() - 1;
        if nums.len() <= 2 {
            return nums[l].min(nums[r]);
        }
        let mut m = l + ((r - l) / 2);
        while l < r {
            if nums[m] < nums[r] {
                r = m;
            } else {
                l = m + 1;
            }
            m = l + ((r - l) / 2);
        }
        nums[l]
    }
}
