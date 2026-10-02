use std::collections::HashSet;

impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let set: HashSet<&i32> = HashSet::from_iter(nums.iter());
        set.len() < nums.len()
    }
}
