use crate::trees::utils::*;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn calc_diameter(max_diameter: &mut i32, head: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match head {
                Some(h) => {
                    let h_cloned = h.clone();
                    let h_cloned_data = h_cloned.borrow_mut();
                    let height_left = calc_diameter(max_diameter, h_cloned_data.left.clone());
                    let height_right = calc_diameter(max_diameter, h_cloned_data.right.clone());
                    *max_diameter = (height_left + height_right).max(*max_diameter);
                    height_left.max(height_right) + 1
                }
                _ => 0,
            }
        }
        let max_diameter: &mut i32 = &mut 0;
        calc_diameter(max_diameter, root);
        *max_diameter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let tree = vec_to_binary_tree(vec![Some(1), Some(2), Some(3)]);
        let result = Solution::diameter_of_binary_tree(tree);
        assert_eq!(result, 2 as i32)
    }

    #[test]
    fn basic_test_2() {
        let tree = vec_to_binary_tree(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let result = Solution::diameter_of_binary_tree(tree);
        assert_eq!(result, 3 as i32)
    }
}
