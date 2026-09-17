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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dum: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
        let mut pt1 = &l1;
        let mut pt2 = &l2;
        let mut chg = &mut dum;
        let mut carry: i32 = 0;
        while let (Some(node1), Some(node2)) = (pt1.as_ref(), pt2.as_ref()) {
            let sum = node1.val + node2.val + carry;
            carry = sum / 10;
            chg = &mut chg.as_mut().unwrap().next;
            *chg = Some(Box::new(ListNode::new(sum % 10)));
            pt1 = &node1.next;
            pt2 = &node2.next;
        }
        let mut then = if pt1.is_some() { pt1 } else { pt2 };
        while let Some(left) = then.as_ref() {
            let sum = left.val + carry;
            carry = sum / 10;
            chg = &mut chg.as_mut().unwrap().next;
            *chg = Some(Box::new(ListNode::new(sum % 10)));
            then = &left.next;
        }
        if carry > 0 {
            chg = &mut chg.as_mut().unwrap().next;
            *chg = Some(Box::new(ListNode::new(carry)));
        }

        dum.unwrap().next
    }
}
