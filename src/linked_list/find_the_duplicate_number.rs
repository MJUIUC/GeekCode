struct Solution {}
impl Solution {
    pub fn find_duplicate_naive(nums: Vec<i32>) -> i32 {
        for (_, &number) in nums.iter().enumerate() {
            let mut seen = 0;
            for i in 0..nums.len() {
                if number == nums[i] {
                    seen += 1;
                }

                if seen > 1 {
                    return number;
                }
            }
        }

        0
    }

    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // problem constraints lead to a linked-list cycle detection algorithm.
        // it works on a mathematical proof, so pretty much must be memorized.
        let mut slow: i32 = nums[0];
        let mut fast: i32 = nums[0];

        // first loop performs actual cycle detection
        loop {
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];

            if slow == fast {
                break;
            }
        }

        // second loop finds cycle entrance
        slow = nums[0];
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        fast
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test_1() {
        let list = vec![3, 1, 3, 4, 2];
        let result = Solution::find_duplicate(list);
        assert_eq!(result, 3)
    }

    #[test]
    fn basic_test_2() {
        let list = vec![1, 3, 4, 2, 2];
        let result = Solution::find_duplicate(list);
        assert_eq!(result, 2)
    }
}
