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
use std::collections::HashMap;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        
        fn dfs (root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>) -> (bool, bool, Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = root.clone() {
                let node = node.borrow();
                let (lp, lq, lanc) = dfs(node.left.clone(), p.clone(), q.clone());
                let (rp, rq, ranc) = dfs(node.right.clone(), p.clone(), q.clone());
                let anc = if !lanc.is_some() {
                    if !ranc.is_some() {
                        None 
                    } else {
                        ranc
                    }
                } else {
                    lanc
                };
                let mut res = (lp | rp, lq | rq, anc);
                if res.0 && res.1 && !res.2.is_some() {
                    res.2 = root;
                    return res;
                }
                else if node.val == p.unwrap().borrow().val {
                    res.0 = true;
                    if res.1 && !res.2.is_some() {
                        res.2 = root;
                    }
                }
                else if node.val == q.unwrap().borrow().val {
                    res.1 = true;
                    if res.0 && !res.2.is_some() {
                        res.2 = root;
                    }
                }
                return res;
            }
            return (false, false, None);
        }
        let guh = dfs(root.clone(), p, q);
        if !guh.2.is_some() {
            return root;
        }
        guh.2
    }
}
