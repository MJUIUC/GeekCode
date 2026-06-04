use crate::linked_list::utils::*;

struct Solution {}
impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        // find the middle with slow/fast pointers (immutable)
        let mut fast = &*head;
        let mut steps = 0;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
            steps += 1;
        }

        // walk to the middle mutably and take the second half
        let mut cur = &mut *head;
        for _ in 0..steps {
            cur = &mut cur.as_mut().unwrap().next;
        }
        let second_half = cur.as_mut().unwrap().next.take();

        // reverse the second half (same pattern as reverse_linked_list)
        let mut prev = None;
        let mut curr = second_half;
        while curr.is_some() {
            let mut temp = curr.unwrap();
            curr = temp.next.take();
            temp.next = prev;
            prev = Some(temp);
        }
        let mut reversed = prev;

        // splice reversed second half into the first half
        let mut cur = &mut *head;
        while cur.is_some() && reversed.is_some() {
            if let Some(node) = cur {
                let rest = node.next.take();
                let mut rev_node = reversed.take().unwrap();
                reversed = rev_node.next.take();
                rev_node.next = rest;
                node.next = Some(rev_node);
                cur = &mut node.next.as_mut().unwrap().next;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorder_even() {
        let mut list = vec_to_single_linked_list(vec![1, 2, 3, 4]);
        Solution::reorder_list(&mut list);
        assert_eq!(list, vec_to_single_linked_list(vec![1, 4, 2, 3]));
    }

    #[test]
    fn test_reorder_odd() {
        let mut list = vec_to_single_linked_list(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut list);
        assert_eq!(list, vec_to_single_linked_list(vec![1, 5, 2, 4, 3]));
    }

    #[test]
    fn test_reorder_single() {
        let mut list = vec_to_single_linked_list(vec![1]);
        Solution::reorder_list(&mut list);
        assert_eq!(list, vec_to_single_linked_list(vec![1]));
    }
}
