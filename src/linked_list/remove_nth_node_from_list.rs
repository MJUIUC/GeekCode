use crate::linked_list::utils::*;

struct Solution {}
impl Solution {
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut head = head;

        let mut cur = head.as_ref();
        let mut list_len: i32 = 0;

        while let Some(node) = cur {
            cur = node.next.as_ref();
            list_len += 1;
        }

        // removal cases
        // 1. nth node at the start of the list
        // 2. nth node has a next that is not none
        // 3. nth node is the end of the list

        let target = list_len - n;

        if target == 0 {
            return head.unwrap().next;
        }

        // println!("{:?}", list_len);
        let mut left = head.as_mut().unwrap();

        // stop in front of the nth node to remove
        for _ in 0..(list_len - n - 1) {
            left = left.next.as_mut().unwrap()
        }

        let removed = left.next.take();
        left.next = removed.unwrap().next;

        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let list = vec_to_linked_list(vec![1, 2, 3, 4, 5]);
        let answer = Solution::remove_nth_from_end(list, 2 as i32);
        assert_eq!(answer, vec_to_linked_list(vec![1, 2, 3, 5]));
    }
}
