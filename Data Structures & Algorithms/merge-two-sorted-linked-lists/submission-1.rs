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

impl Solution {
    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = Box::new(ListNode::new(0));
        let mut iter = &mut res;
        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
            if l1.val < l2.val {
                iter.next = list1;
                iter = iter.next.as_mut().unwrap();
                list1 = iter.next.take();
            } else if l2.val <= l1.val {
                iter.next = list2;
                iter = iter.next.as_mut().unwrap();
                list2 = iter.next.take();
            }
        }
        iter.next = if list1.is_some() { list1 } else { list2 };
        res.next
    }
}
