use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones: BinaryHeap<i32> = BinaryHeap::from(stones);
        while let Some(heavy) = stones.pop() {
            if let Some(light) = stones.pop() {
                if heavy - light > 0 {
                    stones.push(heavy - light);
                }
            } else { return heavy; }
        }
        0
    }
}
