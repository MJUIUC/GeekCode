use crate::trees::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let node_vals: &mut Vec<i32> = &mut vec![];

        fn dfs(r: Option<Rc<RefCell<TreeNode>>>, node_vals: &mut Vec<i32>) {
            if let Some(r) = r {
                let r_ptr = r.borrow();
                dfs(r_ptr.left.clone(), node_vals);
                node_vals.push(r_ptr.val);
                dfs(r_ptr.right.clone(), node_vals);
            }
        }

        dfs(root, node_vals);

        let mut ans = 0;

        for i in 0..node_vals.len() {
            if i == (k - 1) as usize {
                ans = node_vals[i];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let root = vec_to_binary_tree(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);

        let ans = Solution::kth_smallest(root, 3);

        assert_eq!(ans, 3)
    }
}
