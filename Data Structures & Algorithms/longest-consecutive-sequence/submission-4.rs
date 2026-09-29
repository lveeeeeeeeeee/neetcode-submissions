use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut ml = 0;
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for num in nums {
            let mut sum = 1;
            let curr = {*map.entry(num).or_insert(0)};
            if curr == 0 {
                {map.entry(num).and_modify(|r| {*r = 1});}
                let left_opt = {*map.entry(num-1).or_insert(0)};
                let right_opt = {*map.entry(num+1).or_insert(0)};
                match (left_opt, right_opt) {
                    (left, right) if left != 0 && right != 0 => {
                        sum = 1 + left + right;
                        map.entry(num - left).and_modify(|r| {*r = sum});
                        map.entry(num + right).and_modify(|r| {*r = sum});
                    },
                    (left, _) if left != 0 => {
                        sum = 1 + left;
                        map.entry(num - left).and_modify(|r| {*r = sum});
                        map.entry(num).and_modify(|r| {*r = sum});
                    },
                    (_, right) if right != 0 => {
                        sum = 1 + right;
                        map.entry(num + right).and_modify(|r| {*r = sum});
                        map.entry(num).and_modify(|r| {*r = sum});
                    },
                    _ => {}
                }
                ml = ml.max(sum);
            }
        }
        ml
    }
}
