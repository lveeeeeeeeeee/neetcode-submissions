use std::cmp::Ordering;

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

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.val.cmp(&other.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if list1 == Option::None {
            if list2 == Option::None { return None; }
            return list2;
        }
        let mut res: Box<ListNode> = Box::new(ListNode::new(0));
        let mut tail = &mut res;
        while let(Some(buh), Some(guh)) = (list1.as_ref(), list2.as_ref()) {
            if buh.val < guh.val {
                tail.next = list1;
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            }
            else {
                tail.next = list2;
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }
        tail.next = if list1.is_some() { list1 } else { list2 };
        res.next
    }
}
