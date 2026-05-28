use std::collections::HashSet;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_sorted = nums.clone();
        nums_sorted.sort();
        let mut found_sums: HashSet<Vec<i32>> = HashSet::new();

        for i in 0..nums_sorted.len() {
            let mut j = 0;
            let mut k = nums_sorted.len() - 1;

            while j < k {
                if i != j && i != k && j != k {
                    let sum = nums_sorted[i] + nums_sorted[j] + nums_sorted[k];
                    let mut num_set = Vec::from([nums_sorted[i], nums_sorted[j], nums_sorted[k]]);
                    num_set.sort();
                    if sum == 0 && !found_sums.contains(&num_set) {
                        found_sums.insert(num_set);
                    } else if sum < 0 {
                        j += 1;
                    } else {
                        k -= 1;
                    }
                } else {
                    if i == j {
                        j += 1;
                    } else if i == k {
                        k -= 1;
                    }
                }
            }
        }
        found_sums.into_iter().collect()
    }
}
