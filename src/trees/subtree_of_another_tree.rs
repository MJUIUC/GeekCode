use crate::trees::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        fn is_identical(
            p: Option<Rc<RefCell<TreeNode>>>,
            s: Option<Rc<RefCell<TreeNode>>>,
        ) -> bool {
            match (p, s) {
                (Some(p), Some(s)) => {
                    let p = p.borrow();
                    let s = s.borrow();
                    p.val == s.val
                        && is_identical(p.left.clone(), s.left.clone())
                        && is_identical(p.right.clone(), s.right.clone())
                }
                (None, None) => true,
                _ => false,
            }
        }

        match (root, sub_root) {
            (Some(root), Some(sub_root)) => {
                let root_data = root.borrow();

                if is_identical(Some(root.clone()), Some(sub_root.clone())) {
                    return true;
                }

                Self::is_subtree(root_data.left.clone(), Some(sub_root.clone()))
                    || Self::is_subtree(root_data.right.clone(), Some(sub_root.clone()))
            }
            (_, None) => true,
            (None, Some(_)) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let tree = vec_to_binary_tree(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let subtree = vec_to_binary_tree(vec![Some(4), Some(1), Some(2)]);

        assert_eq!(true, Solution::is_subtree(tree, subtree))
    }

    #[test]
    fn basic_test_2() {
        let tree = vec_to_binary_tree(vec![Some(1), Some(1)]);
        let subtree = vec_to_binary_tree(vec![Some(1)]);

        assert_eq!(true, Solution::is_subtree(tree, subtree))
    }
}
