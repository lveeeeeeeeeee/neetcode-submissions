impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(left: &[i32], curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, len: usize) {
            if curr.len() >= len {
                res.push(curr.clone());
                return;
            }
            for i in 0..left.len() {
                let mut emit = Vec::from(&left[..i]);
                emit.extend_from_slice(&left[i+1..]);
                let emit = emit.into_boxed_slice();
                curr.push(left[i]);
                dfs(&emit, curr, res, len);
                curr.pop();
            }
        }

        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut curr: Vec<i32> = Vec::new();
        let len = nums.len();
        let nums = nums.into_boxed_slice();
        dfs(&nums,
        &mut curr,
        &mut res,
        len
        );
        res
    }
}
