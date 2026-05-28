struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let char_arr: Vec<char> = s
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect();

        if char_arr.is_empty() {
            return true;
        }

        // if char_arr.len() == 0 { return true; }
        // if char_arr.len() == 1 { return false; }

        // println!("{:?}", char_arr);

        let mut right = char_arr.len() - 1;
        let mut left = 0;

        while left < right {
            if char_arr[left] != char_arr[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
