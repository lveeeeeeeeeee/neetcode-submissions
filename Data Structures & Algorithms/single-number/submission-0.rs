impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut xor: i32 = 0;
        for num in nums {
            xor ^= num;
        }
        xor
    }
}
