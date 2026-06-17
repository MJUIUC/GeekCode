use crate::trees::utils::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = vec![];
        let mut iter_queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

        if let Some(root) = root {
            iter_queue.push_back(root.clone());

            let mut level = 0;

            while !iter_queue.is_empty() {
                let queue_len = iter_queue.len();
                answer.push(vec![]);

                for _ in 0..queue_len {
                    if let Some(cur_node) = iter_queue.pop_front() {
                        let cur_node_data = cur_node.borrow();
                        answer[level].push(cur_node_data.val);

                        if let Some(cur_left_child) = cur_node_data.left.clone() {
                            iter_queue.push_back(cur_left_child);
                        }

                        if let Some(cur_right_child) = cur_node_data.right.clone() {
                            iter_queue.push_back(cur_right_child);
                        }
                    }
                }
                level += 1
            }
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let root = vec_to_binary_tree(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);

        let result = Solution::level_order(root);
        let answer = vec![vec![3], vec![9, 20], vec![15, 7]];

        assert_eq!(result, answer);
    }
}
