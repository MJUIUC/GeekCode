use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut longest_set: i32 = 0;

        for num in &num_set {
            if !num_set.contains(&(num - 1)) {
                let mut len = 1;
                let mut val = num + 1;
                while num_set.contains(&val) {
                    len += 1;
                    val += 1;
                }
                longest_set = longest_set.max(len);
            }
        }

        longest_set
    }
}
