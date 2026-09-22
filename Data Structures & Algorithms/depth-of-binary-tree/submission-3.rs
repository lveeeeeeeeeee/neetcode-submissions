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

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = Vec::new();
        stack.push((root, 1));
        let mut depth = 0;
        while let Some((node_option, curr)) = stack.pop() {
            if let Some(node) = node_option {
                let node = node.borrow();
                depth = depth.max(curr);
                stack.push((node.left.clone(), curr+1));
                stack.push((node.right.clone(), curr+1));
            }
        }
        depth
    }
}
