impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn dfs(nums: &Vec<i32>, curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, i: usize, sum: i32, t: i32, skipped: bool) {
            if i >= nums.len() {
                return;
            }
            if skipped && nums[i] == nums[i-1] {
                dfs(nums, curr, res, i+1, sum, t, true);
                return;
            } else if i == nums.len() - 1 && sum + nums[i] == t {
                curr.push(nums[i]);
                res.push(curr.clone());
                curr.pop();
                return;
            }
            if sum >= t {
                if sum == t {
                    // println!("");
                    res.push(curr.clone());
                }
                return;
            }
            if nums[i] > t {
                return;
            }
            curr.push(nums[i]);
            dfs(nums, curr, res, i+1, sum + nums[i], t, false);
            curr.pop();
            dfs(nums, curr, res, i+1, sum, t, true);
        }
        
        candidates.sort_unstable();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut curr: Vec<i32> = Vec::new();
        dfs(&candidates, &mut curr, &mut res, 0, 0, target, false);
        res
    }
}
