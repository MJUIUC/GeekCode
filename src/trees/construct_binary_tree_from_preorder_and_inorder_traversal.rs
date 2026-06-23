use crate::trees::utils::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        fn build_tree_helper(
            preorder_ptr: &mut usize,
            left: i32,
            right: i32,
            preorder: &Vec<i32>,
            inorder_map: &HashMap<i32, usize>,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if left > right {
                return None;
            }

            let root_val = preorder[*preorder_ptr];
            let new_root = Rc::new(RefCell::new(TreeNode::new(root_val)));

            *preorder_ptr += 1;

            new_root.borrow_mut().left = build_tree_helper(
                preorder_ptr,
                left,
                inorder_map
                    .get(&root_val)
                    .expect("value should always exist")
                    .clone() as i32
                    - 1,
                preorder,
                inorder_map,
            );

            new_root.borrow_mut().right = build_tree_helper(
                preorder_ptr,
                (inorder_map
                    .get(&root_val)
                    .expect("value should always exist")
                    .clone()
                    + 1) as i32,
                right,
                preorder,
                inorder_map,
            );

            Some(new_root)
        }

        let mut inorder_map: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in inorder.iter().enumerate() {
            inorder_map.insert(num, i);
        }

        let preorder_ptr = &mut 0;
        build_tree_helper(
            preorder_ptr,
            0,
            (preorder.len() - 1) as i32,
            &preorder,
            &inorder_map,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trees::same_tree::Solution as SameTreeSolve;

    #[test]
    fn basic_test_1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];

        let build_tree_result = Solution::build_tree(preorder, inorder);

        let fixture = vec_to_binary_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);

        let ans = SameTreeSolve::is_same_tree(build_tree_result, fixture);

        assert!(ans)
    }
}
