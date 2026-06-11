use crate::trees::utils::*;
use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(p), Some(q)) => {
                let p_data = p.borrow();
                let q_data = q.borrow();
                if p_data.val != q_data.val {
                    return false;
                }
                Self::is_same_tree(p_data.left.clone(), q_data.left.clone())
                    && Self::is_same_tree(p_data.right.clone(), q_data.right.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }
}
