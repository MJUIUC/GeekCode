use std::collections::HashMap;
/**
*
Given a string s, find the length of the longest substring without duplicate characters.

Example 1:

Input: s = "abcabcbb"
Output: 3
Explanation: The answer is "abc", with the length of 3. Note that "bca" and "cab" are also correct answers.
Example 2:

Input: s = "bbbbb"
Output: 1
Explanation: The answer is "b", with the length of 1.
Example 3:

Input: s = "pwwkew"
Output: 3
Explanation: The answer is "wke", with the length of 3.
Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.


Constraints:

0 <= s.length <= 5 * 104
s consists of English letters, digits, symbols and spaces.
*/
use std::collections::HashSet;

struct Solution {}
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let char_vec: Vec<char> = s.chars().collect();
        let mut max_substring_len = 0;
        let mut i = 0;

        while i < s.len() {
            let mut set = HashSet::new();
            let mut j = i;
            while j < s.len() && !set.contains(&char_vec[j]) {
                set.insert(char_vec[j]);
                j += 1;
            }

            max_substring_len = max_substring_len.max(set.len());
            i += 1;
        }

        max_substring_len as i32
    }

    pub fn length_of_longest_substring_revised(s: String) -> i32 {
        let char_vec: Vec<char> = s.chars().collect();
        let mut char_map: HashMap<char, usize> = HashMap::new();
        let mut max_substring_len = 0;
        let mut i = 0;
        let mut j = 0;

        while j < char_vec.len() {
            if let Some(last_seen_char_index) = char_map.get(&char_vec[j]) {
                if *last_seen_char_index >= i {
                    i = last_seen_char_index + 1;
                }
            }

            max_substring_len = max_substring_len.max(j - i + 1);
            char_map.insert(char_vec[j], j);
            j += 1;
        }

        max_substring_len as i32
    }
}

#[cfg(test)]
pub mod tests {

    use super::*;

    #[test]
    fn basic_test_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_revised("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn basic_test_2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring_revised("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn basic_test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring_revised("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn basic_test_4() {
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring_revised(" ".to_string()),
            1
        );
    }

    #[test]
    fn basic_test_5() {
        assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
        assert_eq!(
            Solution::length_of_longest_substring_revised("dvdf".to_string()),
            3
        );
    }
}
