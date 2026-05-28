use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq = HashMap::new();

    for num in nums {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut freq_vec: Vec<(i32, i32)> = freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    freq_vec
        .into_iter()
        .take(k as usize)
        .map(|(num, _)| num)
        .collect()
}
