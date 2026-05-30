/**
Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.

In other words, return true if one of s1's permutations is the substring of s2.



Example 1:

Input: s1 = "ab", s2 = "eidbaooo"
Output: true
Explanation: s2 contains one permutation of s1 ("ba").
Example 2:

Input: s1 = "ab", s2 = "eidboaoo"
Output: false


Constraints:

1 <= s1.length, s2.length <= 104
s1 and s2 consist of lowercase English letters.
 */
use std::collections::HashMap;
struct Solution {}
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1_len = s1.len();
        let s2_len = s2.len();
        let mut s1_map: HashMap<char, i32> = HashMap::new();
        let s2_chars: Vec<char> = s2.chars().collect();

        for c in s1.chars() {
            *s1_map.entry(c).or_insert(0) += 1
        }

        let mut i = 0;
        let mut j = 0;

        let mut window_map: HashMap<char, i32> = HashMap::new();

        while j < s2_len {
            *window_map.entry(s2_chars[j]).or_insert(0) += 1;

            if (j - i + 1) == s1_len {
                if window_map == s1_map {
                    return true;
                }
                if let Some(c_freq) = window_map.get_mut(&s2_chars[i]) {
                    *c_freq -= 1;
                    if *c_freq == 0 {
                        window_map.remove(&s2_chars[i]);
                    }
                }
                i += 1;
            }
            j += 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_testcase_1() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
            true
        );
    }

    #[test]
    fn basic_testcase_2() {
        assert_eq!(
            Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
            false
        );
    }

    #[test]
    fn basic_testcase_3() {
        assert_eq!(
            Solution::check_inclusion("a".to_string(), "eidboaoo".to_string()),
            true
        );
    }

    #[test]
    fn basic_testcase_4() {
        assert_eq!(
            Solution::check_inclusion("dabo".to_string(), "eidboaoo".to_string()),
            true
        );
    }
}
