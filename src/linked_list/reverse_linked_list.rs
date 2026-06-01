use crate::linked_list::utils::ListNode;

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr: Option<Box<ListNode>> = head;

        while curr.is_some() {
            let mut temp = curr.unwrap();
            curr = temp.next.take();
            temp.next = prev;
            prev = Some(temp);
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked_list::utils::vec_to_linked_list;

    #[test]
    fn test_reverse_multiple() {
        assert_eq!(
            Solution::reverse_list(vec_to_linked_list(vec![1, 2, 3, 4, 5])),
            vec_to_linked_list(vec![5, 4, 3, 2, 1])
        );
    }

    #[test]
    fn test_reverse_single() {
        assert_eq!(
            Solution::reverse_list(vec_to_linked_list(vec![1])),
            vec_to_linked_list(vec![1])
        );
    }

    #[test]
    fn test_reverse_empty() {
        assert_eq!(Solution::reverse_list(None), None);
    }
}
