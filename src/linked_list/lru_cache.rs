use std::cell::RefCell;
use std::rc::Rc;

use std::collections::HashMap;

use crate::linked_list::utils::BDRListNode;

struct LRUCache {
    capacity: i32,
    _map: HashMap<i32, Rc<RefCell<BDRListNode>>>, // key to node pointer
    _list_head: Rc<RefCell<BDRListNode>>,
    _list_tail: Rc<RefCell<BDRListNode>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Rc::new(RefCell::new(BDRListNode::new(i32::MIN, -1)));
        let tail = Rc::new(RefCell::new(BDRListNode::new(i32::MAX, -1)));

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());

        Self {
            capacity,
            _map: HashMap::new(),
            _list_head: head,
            _list_tail: tail,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // should move node to front of linked list here
        let mut _next = None;
        let mut _prev = None;

        let cur_node = self._map.get(&key);

        if let Some(node_ptr) = cur_node.clone() {
            _next = node_ptr.borrow().next.clone();
            _prev = node_ptr.borrow().prev.clone();
        } else {
            return -1;
        }

        // removes node from its spot in the list
        _next.as_ref().unwrap().borrow_mut().prev = _prev.clone();
        _prev.as_ref().unwrap().borrow_mut().next = _next.clone();

        // now insert the node to the front of the list
        let rest_of_list = self._list_head.borrow().next.clone();

        // swap pointers on right side of node
        rest_of_list.as_ref().unwrap().borrow_mut().prev = Some(cur_node.unwrap().clone());
        cur_node.unwrap().borrow_mut().next = rest_of_list.clone();
        // swap pointers on left side of node
        cur_node.as_ref().unwrap().borrow_mut().prev = Some(self._list_head.clone());
        self._list_head.borrow_mut().next = Some(cur_node.unwrap().clone());

        cur_node.as_ref().unwrap().borrow().val.1
    }

    fn put(&mut self, key: i32, value: i32) {
        let new_node = Rc::new(RefCell::new(BDRListNode::new(key, value)));

        // always insert or update a new entry
        self._map.insert(key, new_node.clone());

        // move the new node to the front of the list
        let rest_of_list = self._list_head.borrow().next.clone();
        new_node.as_ref().borrow_mut().next = rest_of_list.clone();

        rest_of_list.as_ref().unwrap().borrow_mut().prev = Some(new_node.clone());
        self._list_head.borrow_mut().next = Some(new_node.clone());

        // eval eviction after insertion
        if self._map.len() > self.capacity as usize {
            let lru_node = self._list_tail.borrow().prev.clone();
            // remove node from map
            self._map
                .remove_entry(&lru_node.as_ref().unwrap().borrow().val.0);
            // free node from list
            let prev = lru_node.as_ref().unwrap().borrow().prev.clone();
            prev.as_ref().unwrap().borrow_mut().next = Some(self._list_tail.clone());
            // set tail prev to prev
            self._list_tail.borrow_mut().prev = prev;
        }
    }
}
