impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(l: usize, a: &Vec<i32>, r: &mut Vec<Vec<i32>>, c: &mut Vec<i32>) {
            let mut last: Option<i32> = None;
            if c.len() >= a.len() {
                return;
            }
            for i in l..a.len() {
                if let Some(picked) = last && picked == a[i] {
                    continue;
                } else {
                    c.push(a[i]);
                    r.push(c.clone());
                    dfs(i+1, a, r, c);
                    c.pop();
                    last = Some(a[i]);
                }
            }
        }
        
        let mut res: Vec<Vec<i32>> = Vec::new();
        // sort the array so that it is easier to know which items we haven't picked yet
        nums.sort_unstable();
        // empty set is always a subset of a set so we include it
        res.push(Vec::new());
        let mut curr: Vec<i32> = Vec::new();
        dfs(0, &nums, &mut res, &mut curr);
        res
    }
}