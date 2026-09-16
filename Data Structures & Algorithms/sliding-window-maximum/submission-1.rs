use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // window is updated LIFO-style
        // no easy retrieval of max just by iterating 
        // need to replace the max if it goes out of the window with a value that would be the next max but within the windows borders
        // need to remove old elements
        // need to replace the max if met a greater number

        let k = k as usize;
        let mut ind_deq: VecDeque<usize> = VecDeque::with_capacity(k);
        let mut res: Vec<i32> = Vec::with_capacity(nums.len() - k + 1);
        let mut r: usize = 0;
        let mut l: usize = 0;
        while r < nums.len() {
            while let Some(&back) = ind_deq.back() {
                if nums[r] >= nums[back] {
                    ind_deq.pop_back();
                }
                else {
                    break;
                }
            }
            ind_deq.push_back(r);
            if (r - l) + 1 >= k {
                let newmax = ind_deq.pop_front().unwrap();
                if l <= newmax {
                    ind_deq.push_front(newmax);
                }
                res.push(nums[*ind_deq.front().unwrap()]);
                l += 1;
            }
            else {
                // println!("hey {}, r={}", (r - l) + 1, r);
                r += 1;
            }
        }
        // println!("{:?}", &ind_deq);
        res
    }
}
