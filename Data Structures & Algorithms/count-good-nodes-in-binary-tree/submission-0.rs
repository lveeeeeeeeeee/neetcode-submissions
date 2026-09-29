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
use std::collections::HashSet;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = Vec::new();
        let mut count = 0;
        let good_start = root.as_ref().unwrap().borrow().val;
        stack.push((good_start, root));
        while let Some((good, node_opt)) = stack.pop() {
            if let Some(node) = node_opt {
                let node = node.borrow();
                if node.val >= good {
                    count += 1;
                }
                let good = good.max(node.val);
                if node.left.is_some() {
                    stack.push((good, node.left.clone()));
                }
                if node.right.is_some() {
                    stack.push((good, node.right.clone()));
                }
            }
        }
        count
    }
}
