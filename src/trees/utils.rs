use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// A binary tree node, matching the structure LeetCode uses for tree problems.
///
/// The type of each child is `Option<Rc<RefCell<TreeNode>>>` — here's why each layer matters:
/// - `Option` — a child can be `None` (null), since not every node has two children.
/// - `Rc` (reference-counted pointer) — shared ownership so multiple parts of the
///   tree (or a working queue) can hold a reference to the same node.
/// - `RefCell` — interior mutability, letting us modify fields (e.g. attaching children)
///   even through a shared `&` reference.

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

/// Builds a binary tree from a level-order `Vec<i32>`, matching LeetCode's input format.
/// `-1` is used as the null sentinel (since LeetCode uses `null`).
///
/// The algorithm is BFS: a queue tracks which nodes still need children assigned.
/// For each node popped from the queue, the next two values in the vec are its left
/// and right children. Non-null children get enqueued so their own children are
/// assigned later — this mirrors the level-by-level order the vec stores them in.
///
/// Example: `vec![4, 2, 7, 1, 3]` produces:
///
///        4
///       / \
///      2   7
///     / \
///    1   3
pub fn vec_to_binary_tree(v: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if v.is_empty() || v[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(v[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(Rc::clone(&root));

    let mut i = 1;
    while i < v.len() {
        let node = queue.pop_front().unwrap();

        // Assign left child
        if v[i].is_some() {
            let left = Rc::new(RefCell::new(TreeNode::new(v[i].unwrap())));
            node.borrow_mut().left = Some(Rc::clone(&left));
            queue.push_back(left);
        }
        i += 1;

        // Assign right child
        if i < v.len() && v[i].is_some() {
            let right = Rc::new(RefCell::new(TreeNode::new(v[i].unwrap())));
            node.borrow_mut().right = Some(Rc::clone(&right));
            queue.push_back(right);
        }
        i += 1;
    }

    Some(root)
}
