impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let (mut w_f, mut s_f, mut w_l, mut s_l) = (0, 0, 0, 0);
        for i in 0..nums.len() {
            if i != 0 {
                s_f = w_f.max(s_f + nums[i]);
                (w_f, s_f) = (s_f, w_f);
            }
            if i != nums.len() - 1 {
                s_l = w_l.max(s_l + nums[i]);
                (w_l, s_l) = (s_l, w_l);
            }
        }

        w_l.max(s_l.max(w_f.max(s_f)))
    }
}