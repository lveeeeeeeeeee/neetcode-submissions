impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res: Vec<Vec<i32>> = vec![];
        for i in 0..nums.len()-1 {
            if nums[i] > 0 {
                break;
            }
            let target: i32 = nums[i];
            if i > 0 && target == nums[i-1] {
                continue;
            }
            let mut l: usize = i+1;
            let mut r: usize = nums.len() - 1;
            while r > l {
                if target + nums[l] + nums[r] > 0 {
                    r -= 1;
                }
                else if target + nums[l] + nums[r] < 0 {
                    l += 1;
                }
                else {
                    res.push(vec![target, nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while nums[l-1] == nums[l] && l < r {
                        l += 1;
                    }
                }
            }
        }
        res
    }
}
