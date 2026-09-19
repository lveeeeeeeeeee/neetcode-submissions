impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0usize;
        let mut r = nums.len() - 1;
        if nums.len() == 1 {
            if nums[0] == target {
                return 0;
            }
            return -1;
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
        let moves = l;
        let mut l = 0;
        let mut r = moves;
        while l < r {
            m = l + ((r - l) / 2);
            if nums[m] < target {
                l = m + 1;
            }
            else if nums[m] == target {
                return m as i32;
            }
            else {
                r = m;
            }
        }
        if nums[l] == target { return l as i32; }
        let mut l = moves;
        let mut r = nums.len() - 1;
        while l < r {
            m = l + ((r - l) / 2);
            if nums[m] < target {
                l = m + 1;
            }
            else if nums[m] == target {
                return m as i32;
            }
            else {
                r = m;
            }
        }
        if nums[l] == target { return l as i32; }
        -1
    }
}
