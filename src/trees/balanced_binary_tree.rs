use crate::trees::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn tree_height(head: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match head {
                Some(h) => {
                    let h_data = h.borrow_mut();
                    let tree_height_left = tree_height(h_data.left.clone());
                    let tree_height_right = tree_height(h_data.right.clone());

                    tree_height_left.max(tree_height_right) + 1
                }
                _ => 0,
            }
        }

        if let Some(r) = root {
            let r_data = r.borrow();
            let thl = tree_height(r_data.left.clone());
            let thr = tree_height(r_data.right.clone());

            if (thl - thr).abs() > 1 {
                return false;
            }

            Self::is_balanced(r_data.left.clone()) && Self::is_balanced(r_data.right.clone())
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let input_tree = vec_to_binary_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);

        assert_eq!(true, Solution::is_balanced(input_tree))
    }

    #[test]
    fn basic_test_2() {
        let input_tree = vec_to_binary_tree(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);

        assert_eq!(false, Solution::is_balanced(input_tree))
    }
}
