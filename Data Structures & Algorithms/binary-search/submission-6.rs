impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut r: i32 = nums.len() as i32 - 1;
        let mut l: i32 = 0;
        let mut m = 0;
        while l < r {
            m = l + ((r - l) / 2);
            if target > nums[m as usize] {
                l = m + 1;
            }
            else if target < nums[m as usize]{
                r = m - 1;
            }
            else { return m; }
        }
        if l == -1 || r == nums.len() as i32 {
            return -1;
        }
        else if nums[l as usize] != target {
            return -1;
        }
        l
    }
}
