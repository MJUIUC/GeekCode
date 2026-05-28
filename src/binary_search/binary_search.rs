struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0 as usize;
        let mut r = nums.len();

        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[mid] < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        return -1 as i32;
    }
}
