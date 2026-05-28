pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_left = 0;
        let mut max_right = 0;
        let mut water = 0;

        while left < right {
            if height[left] <= height[right] {
                max_left = max_left.max(height[left]);
                water += max_left - height[left];
                left += 1;
            } else {
                max_right = max_right.max(height[right]);
                water += max_right - height[right];
                right -= 1;
            }
        }

        water
    }
}
