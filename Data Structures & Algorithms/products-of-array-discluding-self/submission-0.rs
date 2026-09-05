impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefixes: Vec<i32> = vec![nums[0]; nums.len()];
        let mut suffixes: Vec<i32> = vec![nums[nums.len()-1]; nums.len()];
        let mut result: Vec<i32> = vec![1; nums.len()];
        let mut pp: usize;
        let mut i: usize;
        for i in 1..nums.len() {
            let sp: usize = nums.len() - i;
            suffixes[sp-1] = suffixes[sp] * nums[sp-1]; 
        }
        for pp in 0..nums.len() {
            if (pp == 0) {
                result[pp] = suffixes[pp+1];
            }
            else if (pp == nums.len() - 1) {
                result[pp] = prefixes[pp-1];
            }
            else {
                prefixes[pp] = prefixes[pp-1] * nums[pp];
                result[pp] = prefixes[pp-1] * suffixes[pp+1];
            }
        }
        result
    }
}
