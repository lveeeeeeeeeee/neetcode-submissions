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
