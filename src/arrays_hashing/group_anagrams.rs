use itertools::Itertools;
use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // itterate through strs adding each string to a map<str, vec<str>>
    // the key to the map should be a sorted string
    // during the iteration block, store the unsorted key in a list

    let mut grouped_anagrams: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs.into_iter() {
        let key: String = s.chars().into_iter().sorted().collect();
        grouped_anagrams.entry(key).or_default().push(s);
    }

    return grouped_anagrams.into_values().collect();
}
