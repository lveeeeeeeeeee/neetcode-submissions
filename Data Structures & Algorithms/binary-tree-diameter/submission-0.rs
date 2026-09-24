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

// probably Djikstra's would be okay

// degenerate tree diameter = depth of the tree
// special tree condition: children of the root node are degenerate subtrees
// if condition is satisfied, diameter = sum of depths + 2
// diameter of a tree = max(max(special subtree diameters), max(depth))
// node that has no children has a diameter of 0
// practically this is solved with dynamic programming

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn degenerate(node_option: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            let mut result = (0, 0);
            if let Some(node) = node_option.as_ref() {
                let node_ref = node.borrow();
                result = match (node_ref.left.clone(), node_ref.right.clone()) {
                    (Some(left), Some(right)) => {
                        let im = special(&Some(node.clone()));
                        (im.0, im.1)
                    }
                    (Some(left), None) => {
                        let im = degenerate(&Some(left));
                        (im.0 + 1, im.1)
                    }
                    (None, Some(right)) => {
                        let im = degenerate(&Some(right));
                        (im.0 + 1, im.1)
                    }
                    (None, None) => { (0, 0) }
                }
            }
            result
        }

        fn special(node_option: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            let node = node_option.as_ref();
            let (left, left_special) = degenerate(&node.unwrap().borrow().left);
            let (right, right_special) = degenerate(&node.unwrap().borrow().right);
            ((left+1).max(right+1), left_special.max(right_special.max(left+right+2)))
        }
        
        let result = degenerate(&root);
        result.0.max(result.1)
    }
}
