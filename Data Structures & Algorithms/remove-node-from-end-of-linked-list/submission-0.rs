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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut counter: i32 = 0;
        // separate scope so that we drop a reference
        {
            let mut fast: &Option<Box<ListNode>> = &head;
            while let Some(node_fast) = fast.as_ref() {
                fast = &node_fast.next;
                counter += 1;
            }
        }
        let mut dum = Some(Box::new(ListNode { val: 0, next: head }));
        let mut iter: &mut Option<Box<ListNode>> = &mut dum;
        // counter = length of the list
        // index of last element == length - 1
        // index of element to remove would be length - n
        let mut i = 0;
        let counter = counter;
        while i < counter - n {
            iter = &mut iter.as_mut().unwrap().next;
            i += 1;
        }
        if let Some(mut thing) = iter.as_mut() {
            let gap = thing.next.take();
            thing.next = gap.unwrap().next;
        }
        dum.unwrap().next
    }
}
