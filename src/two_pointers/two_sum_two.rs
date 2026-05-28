impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left_index = 0;
        let mut right_index = numbers.len() - 1;

        while left_index < right_index {
            let cur_sum = numbers[left_index] + numbers[right_index];
            if cur_sum == target {
                return vec![(left_index + 1) as i32, (right_index + 1) as i32];
            } else if cur_sum > target {
                right_index -= 1;
            } else {
                left_index += 1;
            }
        }
    }
}
