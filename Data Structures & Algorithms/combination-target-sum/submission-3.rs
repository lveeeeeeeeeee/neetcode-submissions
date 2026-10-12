impl Solution {
    // pick the biggest number while sum < target
    // then backtrack and select the nums that contribute to the target
    pub fn combination_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();

        // greedy yayyy
        nums.sort_unstable_by_key(|k| -k);
        let mut set: Vec<i32> = Vec::new();
        Solution::dfs(0, &nums, &mut set, 0, target, &mut res);
        res
    }

    fn dfs(mut ind: usize, nums: &Vec<i32>, set: &mut Vec<i32>, mut sum_rn: i32, target: i32, res: &mut Vec<Vec<i32>>) {
        if ind >= nums.len() {
            return;
        }
        if sum_rn > target {
            return;
        } else if sum_rn == target {
            res.push(set.clone());
            return
        } else {
            set.push(nums[ind]);
            Solution::dfs(ind, nums, set, sum_rn + nums[ind], target, res);
            set.pop();
            Solution::dfs(ind+1, nums, set, sum_rn, target, res);
        }
    }
}