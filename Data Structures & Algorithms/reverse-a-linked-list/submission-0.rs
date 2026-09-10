use std::collections::VecDeque;

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

impl ListNode {
    #[inline]
    pub fn new_followed(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {next, val}
    }
}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == Option::None {
            return head;
        }
        let mut head = head;
        let mut prev: Option<Box<ListNode>> = Option::None;
        while let Some(mut curr) = head.take() {
            head = curr.next;
            curr.next = prev;
            prev = Some(curr);
        }

        prev
    }
}
