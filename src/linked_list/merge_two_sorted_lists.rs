use crate::linked_list::utils::ListNode;

struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut previous: Box<ListNode> = Box::new(ListNode::new(-1));
        let mut cur = previous.as_mut();

        let mut p1: Option<Box<ListNode>> = list1;
        let mut p2: Option<Box<ListNode>> = list2;

        // infinite loop
        loop {
            // match on nodes as a reference, Option<&Box<ListNode>>
            match (p1.as_ref(), p2.as_ref()) {
                // p1_ref == &Box<ListNode>
                (Some(p1_ref), Some(p2_ref)) => {
                    // which allows this value comparison
                    if p1_ref.val > p2_ref.val {
                        // and we can still work with p1 and p2
                        cur.next = p2.take();
                        cur = cur.next.as_mut().unwrap();
                        p2 = cur.next.take();
                    } else {
                        cur.next = p1.take();
                        cur = cur.next.as_mut().unwrap();
                        p1 = cur.next.take();
                    }
                }
                _ => break,
            }
        }

        cur.next = if p1.is_some() { p1 } else { p2 };
        previous.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::linked_list::utils::vec_to_linked_list;

    #[test]
    fn test_merge_basic() {
        assert_eq!(
            Solution::merge_two_lists(
                vec_to_linked_list(vec![1, 2, 4]),
                vec_to_linked_list(vec![1, 3, 4])
            ),
            vec_to_linked_list(vec![1, 1, 2, 3, 4, 4])
        );
    }

    #[test]
    fn test_merge_empty_lists() {
        assert_eq!(Solution::merge_two_lists(None, None), None);
    }

    #[test]
    fn test_merge_one_empty() {
        assert_eq!(
            Solution::merge_two_lists(None, vec_to_linked_list(vec![1, 2, 3])),
            vec_to_linked_list(vec![1, 2, 3])
        );
    }
}
