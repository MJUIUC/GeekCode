use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut freq = HashMap::new();

    for c in s.chars() {
        *freq.entry(c).or_insert(0) += 1;
    }

    for c in t.chars() {
        *freq.entry(c).or_insert(0) -= 1;
    }

    return freq.values().all(|&v| v == 0);
}
