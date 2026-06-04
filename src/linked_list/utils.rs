use std::cell::RefCell;
use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn vec_to_single_linked_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &val in vals.iter().rev() {
        let mut node = ListNode::new(val);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

// Definition for bi-directional linked list.
#[derive(Clone, Debug)]
pub struct BDRListNode {
    pub val: (i32, i32),
    pub prev: Option<Rc<RefCell<BDRListNode>>>,
    pub next: Option<Rc<RefCell<BDRListNode>>>,
}

impl BDRListNode {
    #[inline]
    pub fn new(key: i32, val: i32) -> Self {
        BDRListNode {
            val: (key, val),
            prev: None,
            next: None,
        }
    }
}
