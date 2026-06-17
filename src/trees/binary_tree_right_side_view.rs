use crate::trees::utils::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut iter_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        let mut right_side: Vec<i32> = vec![];

        if let Some(root) = root {
            iter_queue.push_back(root);

            while !iter_queue.is_empty() {
                let queue_len = iter_queue.len();

                for i in 0..queue_len {
                    if let Some(cur_node) = iter_queue.pop_front() {
                        let cur_node_data = cur_node.borrow();

                        if let Some(cur_left_child) = cur_node_data.left.clone() {
                            iter_queue.push_back(cur_left_child);
                        }

                        if let Some(cur_right_child) = cur_node_data.right.clone() {
                            iter_queue.push_back(cur_right_child);
                        }

                        if i == queue_len - 1 {
                            right_side.push(cur_node_data.val);
                        }
                    }
                }
            }
        }

        right_side
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let root = vec_to_binary_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            None,
            Some(5),
        ]);

        let result = Solution::right_side_view(root);
        let answer = vec![1, 3, 4, 5];

        assert_eq!(result, answer);
    }

    #[test]
    fn basic_test_2() {
        let root = vec_to_binary_tree(vec![
            Some(1),
            Some(2),
            Some(3),
            None,
            Some(5),
            None,
            Some(4),
        ]);

        let result = Solution::right_side_view(root);
        let answer = vec![1, 3, 4];

        assert_eq!(result, answer);
    }
}
