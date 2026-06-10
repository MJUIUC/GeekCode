use std::cell::RefCell;
use std::rc::Rc;

use crate::trees::utils::*;

struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(r) => {
                let r_clone = r.clone();
                let mut root_data = r_clone.borrow_mut();
                let temp = root_data.left.clone();
                root_data.left = root_data.right.clone();
                root_data.right = temp;
                Self::invert_tree(root_data.left.clone());
                Self::invert_tree(root_data.right.clone());
                Some(r)
            }
            _ => {
                return root;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let tree_root = vec_to_binary_tree(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let inverted_tree = Solution::invert_tree(tree_root);

        let ans_tree_root = vec_to_binary_tree(vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]);

        assert_eq!(inverted_tree, ans_tree_root)
    }
}
