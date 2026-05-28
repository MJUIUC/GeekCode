/**
*
*
There is an integer array nums sorted in ascending order (with distinct values).

Prior to being passed to your function, nums is possibly left rotated at an unknown
index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1],
..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7]
might be left rotated by 3 indices and become [4,5,6,7,0,1,2].

Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

You must write an algorithm with O(log n) runtime complexity.


Example 1:

Input: nums = [4,5,6,7,0,1,2], target = 0
Output: 4
Example 2:

Input: nums = [4,5,6,7,0,1,2], target = 3
Output: -1
Example 3:

Input: nums = [1], target = 0
Output: -1
*
*/
struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let nums_len = nums.len();
        let mut left = 0;
        let mut right = nums_len - 1;

        while left < right {
            let mid = (left + right) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[left] <= nums[mid] {
                if nums[left] <= target && target <= nums[mid] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            } else {
                if nums[mid] <= target && target <= nums[right] {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
        }

        if nums[left] == target {
            left as i32
        } else {
            -1
        }
    }
}
