use std::collections::HashMap;

use itertools::Itertools;

pub struct Solution {
    // need map from { to } and the rest of the parenthesis
}

impl Solution {
    pub fn is_valid(input: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let mut map: std::collections::HashMap<char, char> = HashMap::new();
        map.insert(')', '(');
        map.insert('}', '{');
        map.insert(']', '[');

        for c in input.chars() {
            if stack.len() != 0 && map.keys().contains(&c) && stack.last() == map.get(&c) {
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack.is_empty()
    }
}
