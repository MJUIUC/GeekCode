use crate::trees::utils::*;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn calc_depth(mut depth: i32, head: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match head {
                Some(h) => {
                    let h_cloned = h.clone();
                    let h_cloned_data = h_cloned.borrow_mut();
                    depth += 1;
                    let depth_left = calc_depth(depth, h_cloned_data.left.clone());
                    let depth_right = calc_depth(depth, h_cloned_data.right.clone());
                    depth_left.max(depth_right)
                }
                _ => {
                    return depth;
                }
            }
        }

        calc_depth(0, root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let tree = vec_to_binary_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let answer = Solution::max_depth(tree);

        assert_eq!(answer, 3 as i32)
    }

    #[test]
    fn basic_test_2() {
        let tree = vec_to_binary_tree(vec![Some(1), None, Some(2)]);
        let answer = Solution::max_depth(tree);

        assert_eq!(answer, 2 as i32)
    }
}
