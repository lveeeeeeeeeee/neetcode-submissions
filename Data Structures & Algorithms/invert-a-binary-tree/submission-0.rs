// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

use std::collections::VecDeque;

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if !root.is_some() {
            return root;
        }
        let mut q = VecDeque::new();
        q.push_back(root.clone().unwrap());
        while let Some(curr) = q.pop_front() {
            let mut curr_ref = curr.borrow_mut();
            let left = curr_ref.left.take();
            let right = curr_ref.right.take();
            curr_ref.left = right;
            curr_ref.right = left;
            if let Some(ref next_left) = curr_ref.left {
                q.push_back(next_left.clone());
            }
            if let Some(ref next_right) = curr_ref.right {
                q.push_back(next_right.clone());
            }
        }
        root
    }
}
