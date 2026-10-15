impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut prefix = vec![0; nums.len()+1];
        for i in 0..prefix.len() {
            prefix[i] = if i == 0 { 0 } else { nums[i-1] + prefix[i-1] };
        }
        let mut r = 1;
        let mut l: Option<usize> = None;
        while r < prefix.len() {
            let sum = if l.is_some() { prefix[r] - prefix[l.unwrap()] } else { prefix[r] };
            max = max.max(sum);
            if sum <= 0 {
                l = Some(r);
            }
            r += 1;
        }
        max
    }
}