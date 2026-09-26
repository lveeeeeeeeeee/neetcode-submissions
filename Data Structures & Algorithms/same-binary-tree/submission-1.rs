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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if (p.is_some() && !q.is_some()) || (!p.is_some() && q.is_some()) {
            return false
        }
        if !p.is_some() && !q.is_some() {
            return true;
        }
        let mut queue = VecDeque::new();
        queue.push_back((q, p));
        while let Some((Some(f), Some(s))) = queue.pop_front() {
            let f = f.borrow();
            let s = s.borrow();
            if f.left.is_some() && !s.left.is_some() {
                return false;
            }
            if f.right.is_some() && !s.right.is_some() {
                return false;
            }
            if !f.left.is_some() && s.left.is_some() {
                return false;
            }
            if !f.right.is_some() && s.right.is_some() {
                return false;
            }
            if f.val != s.val {
                return false;
            }
            if let (Some(fleft), Some(sleft)) = (f.left.clone(), s.left.clone()) {
                if fleft.borrow().val != sleft.borrow().val {
                    return false;
                }
                queue.push_back((Some(fleft), Some(sleft)));
            }
            if let (Some(fright), Some(sright)) = (f.right.clone(), s.right.clone()) {
                if fright.borrow().val != sright.borrow().val {
                    return false;
                }
                queue.push_back((Some(fright), Some(sright)));
            }
        }
        true
    }
}
