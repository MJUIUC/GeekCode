use crate::linked_list::utils::*;

struct Solution {}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        fn reverse_list(h: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut reversed: Option<Box<ListNode>> = None;
            let mut cur = h;

            while let Some(node) = cur {
                let mut temp = node;
                cur = temp.next.take();
                temp.next = reversed;
                reversed = Some(temp)
            }

            reversed
        }

        let mut cur_l1 = l1.as_ref();
        let mut cur_l2 = l2.as_ref();
        let mut new_list_head: Option<Box<ListNode>> = None;
        let mut carry: i32 = 0;

        while cur_l1.is_some() || cur_l2.is_some() {
            let mut sum = carry;

            if cur_l1.is_some() {
                sum += cur_l1.unwrap().val;
                cur_l1 = cur_l1.unwrap().next.as_ref();
            }

            if cur_l2.is_some() {
                sum += cur_l2.unwrap().val;
                cur_l2 = cur_l2.unwrap().next.as_ref();
            }

            carry = sum / 10;
            let mut new_node = Box::new(ListNode::new(sum % 10));
            new_node.next = new_list_head;
            new_list_head = Some(new_node);
        }

        if carry > 0 {
            let mut new_node = Box::new(ListNode::new(carry));
            new_node.next = new_list_head;
            new_list_head = Some(new_node);
        }

        reverse_list(new_list_head)
    }

    pub fn add_two_numbers_opt(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut cur_l1 = l1.as_ref();
        let mut cur_l2 = l2.as_ref();
        let mut new_list_head: Box<ListNode> = Box::new(ListNode::new(-1));

        let mut new_list_pointer = &mut new_list_head;
        let mut carry: i32 = 0;

        while cur_l1.is_some() || cur_l2.is_some() {
            let mut sum = carry;

            if cur_l1.is_some() {
                sum += cur_l1.unwrap().val;
                cur_l1 = cur_l1.unwrap().next.as_ref();
            }

            if cur_l2.is_some() {
                sum += cur_l2.unwrap().val;
                cur_l2 = cur_l2.unwrap().next.as_ref();
            }

            carry = sum / 10;
            let new_node = Box::new(ListNode::new(sum % 10));
            new_list_pointer.next = Some(new_node);
            new_list_pointer = new_list_pointer.next.as_mut().unwrap();
        }

        if carry > 0 {
            let new_node = Box::new(ListNode::new(carry));
            new_list_pointer.next = Some(new_node);
            new_list_pointer = new_list_pointer.next.as_mut().unwrap()
        }

        new_list_head.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let l1 = vec_to_single_linked_list(vec![2, 4, 3]);
        let l2 = vec_to_single_linked_list(vec![5, 6, 4]);

        let answer = Solution::add_two_numbers(l1, l2);

        assert_eq!(answer, vec_to_single_linked_list(vec![7, 0, 8]));
    }

    #[test]
    fn basic_test_2() {
        let l1 = vec_to_single_linked_list(vec![9, 2, 8]);
        let l2 = vec_to_single_linked_list(vec![6, 6, 4]);

        let answer = Solution::add_two_numbers(l1, l2);

        assert_eq!(answer, vec_to_single_linked_list(vec![5, 9, 2, 1]));
    }

    #[test]
    fn basic_test_3() {
        let l1 = vec_to_single_linked_list(vec![9, 2, 8]);
        let l2 = None;

        let answer = Solution::add_two_numbers(l1, l2);

        assert_eq!(answer, vec_to_single_linked_list(vec![9, 2, 8]));
    }

    // OPT function

    #[test]
    fn basic_test_1_opt() {
        let l1 = vec_to_single_linked_list(vec![2, 4, 3]);
        let l2 = vec_to_single_linked_list(vec![5, 6, 4]);

        let answer = Solution::add_two_numbers_opt(l1, l2);

        assert_eq!(answer, vec_to_single_linked_list(vec![7, 0, 8]));
    }

    #[test]
    fn basic_test_2_opt() {
        let l1 = vec_to_single_linked_list(vec![9, 2, 8]);
        let l2 = vec_to_single_linked_list(vec![6, 6, 4]);

        let answer = Solution::add_two_numbers_opt(l1, l2);

        assert_eq!(answer, vec_to_single_linked_list(vec![5, 9, 2, 1]));
    }

    #[test]
    fn basic_test_3_opt() {
        let l1 = vec_to_single_linked_list(vec![9, 2, 8]);
        let l2 = None;

        let answer = Solution::add_two_numbers_opt(l1, l2);

        assert_eq!(answer, vec_to_single_linked_list(vec![9, 2, 8]));
    }
}
