// class Solution:
//     def productExceptSelf(self, nums: List[int]) -> List[int]:
//         N = len(nums)
//         ans = [1]*N

//         left = 1
//         for i in range(N):
//             ans[i] = left
//             left *= nums[i]

//         right = 1
//         for i in range(N-1, -1, -1):
//             ans[i] *= right
//             right *= nums[i]

//         return ans
//
struct Solution {}
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];
        let mut left = 1;
        for i in 0..n {
            ans[i] = left;
            left *= nums[i];
        }
        let mut right = 1;
        for i in (0..n).rev() {
            ans[i] *= right;
            right *= nums[i];
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        assert_eq!(Solution::product_except_self(nums), expected);
    }
}
