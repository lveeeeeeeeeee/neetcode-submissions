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
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut q: VecDeque<(Option<Rc<RefCell<TreeNode>>>, usize)> = VecDeque::new();
        q.push_back((root, 1));
        let mut res: Vec<Vec<i32>> = Vec::new();
        while let Some((node_opt, level)) = q.pop_front() {
            if let Some(node) = node_opt {
                if res.len() < level {
                    res.push(Vec::new());
                }
                let node = node.borrow();
                if let Some(ref left) = node.left {
                    q.push_back((Some(left.clone()), level + 1));
                }
                if let Some(ref right) = node.right {
                    q.push_back((Some(right.clone()), level + 1));
                }
                res[level-1].push(node.val);
            }
        }
        res
    }
}
