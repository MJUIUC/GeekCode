use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for (i, &num) in nums.iter().enumerate() {
        for (j, &other) in nums.iter().enumerate().skip(i + 1) {
            if num + other == target {
                return Some((i, j));
            }
        }
    }
    None
}

pub fn two_sum_hash(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return Some((j, i));
        }
        map.insert(num, i);
    }
    None
}
