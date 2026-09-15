// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        let mut slow: *mut ListNode = head;
        let mut fast: *mut ListNode = head;
        // println!("head is {:?}", head);
        let mut res = false;
        unsafe {
            loop {
                let mut steps = 2;
                while steps > 0 {
                    if fast == 0 as *mut ListNode {
                        return false;
                    } else {
                        // println!("{:?}", fast);
                        fast = (*fast).next;
                        steps -= 1;
                    }
                }
                slow = (*slow).next;
                if slow == fast {
                    res = true;
                    break;
                }
            }
        }
        return res;
    }
}
