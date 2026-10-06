use std::collections::{BinaryHeap, VecDeque};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if n == 0 {
            return tasks.len() as i32;
        }
        let mut map: [i32; 26] = [0; 26];
        for elem in tasks {
            map[elem as usize - b'A' as usize] += 1;
        }
        let mut amts = BinaryHeap::with_capacity(26usize);
        for elem in map {
            if elem != 0 {
                amts.push(elem);
            }
        }
        let mut qexec: VecDeque<(i32, i32)> = VecDeque::with_capacity(26usize);
        let mut ans = 0;
        while !qexec.is_empty() || !amts.is_empty() {
            ans += 1;
            if amts.is_empty() {
                ans = qexec.front().unwrap().1;
            } else {
                let peek = amts.pop().unwrap() - 1;
                if peek >= 1 {
                    qexec.push_back((peek, ans + n));
                }
            }
            if let Some(front) = qexec.front() {
                let mut front = *front;
                if front.1 == ans {
                    qexec.pop_front();
                    amts.push(front.0);
                }
            }
        }
        ans
    }
}
