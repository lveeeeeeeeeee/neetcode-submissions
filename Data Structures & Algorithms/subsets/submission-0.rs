use std::collections::VecDeque;

impl Solution {
    fn recursion(i: usize, nums: &Vec<i32>, sub: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if i >= nums.len() {
            res.push(sub.clone());
            return;
        }
        // pick
        sub.push(nums[i]);
        Self::recursion(i + 1, nums, sub, res);
        // drop
        sub.pop();
        // recurse without picking
        Self::recursion(i + 1, nums, sub, res);
    }

    // pascal triangle amt of iterations
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut subset: Vec<i32> = Vec::new();

        Self::recursion(0, &nums, &mut subset, &mut res);
        res
    }
}