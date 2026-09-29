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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut max: i32 = 0;
        let mut res: Vec<i32> = vec![0; 0];
        
        fn dfs(mp: &mut i32, v: &mut Vec<i32>, depth: i32, node_opt: &Option<Rc<RefCell<TreeNode>>>) -> () {
            if let Some(node) = node_opt {
                let node = node.borrow();
                println!("{}", &node.val);
                if node.right.is_some() {
                    println!("node: {}, right: {}", &node.val, &node.right.as_ref().unwrap().borrow().val);
                }
                if node.left.is_some() {
                    println!("node: {}, left: {}", &node.val, &node.left.as_ref().unwrap().borrow().val);
                }
                if depth > *mp {
                    *mp = depth;
                    v.push(node.val);
                }
                dfs(mp, v, depth+1, &node.right);
                dfs(mp, v, depth+1, &node.left);
            }
        }
        dfs(&mut max, &mut res, 1, &root);
        res
    }
}
