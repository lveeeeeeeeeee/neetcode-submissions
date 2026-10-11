use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn max_rec(root: &Option<Rc<RefCell<TreeNode>>>, m: &mut i32) -> (Option<i32>, Option<i32>) {
            if let Some(node) = root {
                let node = node.borrow();
                let node_val = node.val;
                *m = (*m).max(node_val);
                let (l_re, l_unre) = max_rec(&node.left, m);
                let (r_re, r_unre) = max_rec(&node.right, m);
                let mut max_reach: Option<i32> = None;
                let mut curr_unre: Option<i32> = None;
                // reachables
                match (l_re, r_re) {
                    (Some(l), Some(r)) => {
                        max_reach = Some(l.max(r.max(0)));
                        curr_unre = Some(l + r + node_val);
                    },
                    (Some(l), None) => { max_reach = Some(l.max(0)); },
                    (None, Some(r)) => { max_reach = Some(r.max(0)); },
                    _ => {  }
                }
                if let Some(curr) = curr_unre {
                    let mut curr = curr;
                    if let Some(l_u) = l_unre {
                        curr = curr.max(l_u);
                    }
                    if let Some(r_u) = r_unre {
                        curr = curr.max(r_u);
                    }
                    *m = (*m).max(curr);
                    curr_unre = Some(curr);
                }
                if let Some(max) = max_reach {
                    *m = (*m).max(max + node_val);
                    return (Some(max + node_val), curr_unre);
                } else {
                    *m = (*m).max(node_val);
                    return (Some(node_val), curr_unre);
                }
            } else {
                return (Some(0), None);
            } 
       }
       let mut m = root.as_ref().unwrap().borrow().val;
       let (reach, unreach) = max_rec(&root, &mut m);
       m
    }
}
