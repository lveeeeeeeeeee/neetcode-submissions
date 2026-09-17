impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
            if slow == fast {
                break;
            }
        }
        let mut detect = 0;
        while detect != slow {
            slow = nums[slow as usize];
            detect = nums[detect as usize];
        }
        detect
    }
}
