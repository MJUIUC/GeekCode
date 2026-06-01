use crate::linked_list::utils::*;

struct Solution {}
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut slow = &*head;
        let mut fast = &*head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
    }
}
