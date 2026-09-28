// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val)
    }
}

use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut res: Box<ListNode> = Box::new(ListNode::new(0));
        let mut curr: &mut Box<ListNode> = &mut res;
        let mut pq: BinaryHeap<Option<Box<ListNode>>> = BinaryHeap::with_capacity(lists.len());
        for i in 0..lists.len() {
            if let Some(node) = lists[i].clone() {
                pq.push(Some(node));
            }
        }
        while let Some(mut min) = pq.pop() {
            curr.next = min;
            curr = curr.next.as_mut().unwrap();
            min = curr.next.take();
            if min.is_some() {
                pq.push(min);
            }
        }
        res.next
    }
}
