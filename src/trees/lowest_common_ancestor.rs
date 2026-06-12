use crate::trees::utils::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let (Some(r), Some(p), Some(q)) = (root, p, q) {
            let r_data = r.borrow();
            let p_data = p.borrow();
            let q_data = q.borrow();

            let r_val = r_data.val;
            let p_val = p_data.val;
            let q_val = q_data.val;

            if r_val < p_val && r_val < q_val {
                return Self::lowest_common_ancestor(
                    r_data.right.clone(),
                    Some(p.clone()),
                    Some(q.clone()),
                );
            } else if r_val > p_val && r_val > q_val {
                return Self::lowest_common_ancestor(
                    r_data.left.clone(),
                    Some(p.clone()),
                    Some(q.clone()),
                );
            } else {
                return Some(r.clone());
            }
        }

        None
    }

    pub fn lowest_common_ancestor_itterative(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let (Some(r), Some(p), Some(q)) = (root, p, q) {
            let p_data = p.borrow();
            let q_data = q.borrow();

            let p_val = p_data.val;
            let q_val = q_data.val;

            let mut cur_root = Some(r.clone());

            loop {
                if let Some(c_r) = cur_root {
                    let c_r_data = c_r.borrow();
                    let r_val = c_r_data.val;

                    if r_val < p_val && r_val < q_val {
                        cur_root = c_r_data.right.clone();
                    } else if r_val > p_val && r_val > q_val {
                        cur_root = c_r_data.left.clone();
                    } else {
                        return Some(c_r.clone());
                    }
                }
            }
        }

        None
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

        let p = Some(Rc::new(RefCell::new(TreeNode::new(2 as i32))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8 as i32))));

        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(6, result.unwrap().borrow().val)
    }

    #[test]
    fn basic_test_1_itterative() {
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

        let p = Some(Rc::new(RefCell::new(TreeNode::new(2 as i32))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8 as i32))));

        let result = Solution::lowest_common_ancestor_itterative(root, p, q);
        assert_eq!(6, result.unwrap().borrow().val)
    }
}
