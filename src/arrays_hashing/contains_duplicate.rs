pub fn contains_duplicate(nums: &[i32]) -> bool {
    let mut set = std::collections::HashSet::new();
    for &num in nums {
        if !set.insert(num) {
            return true;
        }
    }
    false
}
