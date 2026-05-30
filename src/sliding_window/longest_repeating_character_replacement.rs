/**
*
You are given a string s and an integer k. You can choose any character of the string and change it to any other uppercase English character. You can perform this operation at most k times.

Return the length of the longest substring containing the same letter you can get after performing the above operations.



Example 1:

Input: s = "ABAB", k = 2
Output: 4
Explanation: Replace the two 'A's with two 'B's or vice versa.
Example 2:

Input: s = "AABABBA", k = 1
Output: 4
Explanation: Replace the one 'A' in the middle with 'B' and form "AABBBBA".
The substring "BBBB" has the longest repeating letters, which is 4.
There may exists other ways to achieve this answer too.


Constraints:

1 <= s.length <= 105
s consists of only uppercase English letters.
0 <= k <= s.length
*
*/
use std::collections::HashMap;

struct Solution {}
impl Solution {
    // The idea for this was incorrect and it didn't pass all use cases.
    // It does not at all account for all of the characters of the alphabet.
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0; // last change in variable
        let mut j = 1;

        let mut longest_replacement_substring = 1;
        let mut cur_k = k;

        while j < chars.len() {
            // case where chars[j] is a different letter and potential replacement
            if cur_k < 1 {
                // we are out of substitutions for this window, calculate length and reset window
                longest_replacement_substring = longest_replacement_substring.max(j - i + 1); // +1 to include the current character
                cur_k = k;
                i = j;
            } else if chars[j] != chars[j - 1] {
                // only decrement cur_k if it's greater than or equal to k and there are differing adjacent chars
                longest_replacement_substring = longest_replacement_substring.max(j - i + 1);
                cur_k -= 1;
            }

            j += 1;
        }

        longest_replacement_substring as i32
    }

    // This passes.
    pub fn character_replacement_v2(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut char_frequency_map: HashMap<char, i32> = HashMap::new();

        let mut longest_replacement_substring = 0;

        let mut i = 0;
        let mut j = 0;

        while j < chars.len() {
            char_frequency_map
                .entry(chars[j])
                .and_modify(|freq| *freq += 1)
                .or_insert(1);

            let highest_freq = char_frequency_map.values().max().unwrap();

            if (j as i32 - i as i32 + 1) - highest_freq <= k {
                longest_replacement_substring = longest_replacement_substring.max(j - i + 1);
            } else {
                char_frequency_map
                    .entry(chars[i])
                    .and_modify(|freq| *freq -= 1);
                i += 1;
            }

            j += 1;
        }

        longest_replacement_substring as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_testcase_1() {
        assert_eq!(
            Solution::character_replacement_v2("AABABBA".to_string(), 1),
            4
        );
    }

    // #[test]
    // fn basic_testcase_2() {
    //     assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
    // }

    // #[test]
    // fn basic_testcase_3() {
    //     assert_eq!(Solution::character_replacement("A".to_string(), 1), 1);
    // }

    // #[test]
    // fn basic_testcase_4() {
    //     assert_eq!(Solution::character_replacement("A".to_string(), 0), 1);
    // }

    // #[test]
    // fn basic_testcase_5() {
    //     assert_eq!(Solution::character_replacement("AB".to_string(), 1), 2);
    // }

    #[test]
    fn basic_testcase_6() {
        assert_eq!(Solution::character_replacement_v2("AB".to_string(), 0), 1);
    }
}
