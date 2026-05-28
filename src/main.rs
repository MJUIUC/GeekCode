mod arrays_hashing;
mod binary_search;
mod stack;
mod two_pointers;

use arrays_hashing::contains_duplicate;
use arrays_hashing::encode_and_decode_strings;
use arrays_hashing::product_of_array_except_self;
use arrays_hashing::top_k_freq;
use arrays_hashing::two_sum;
use arrays_hashing::valid_sudoku;

use two_pointers::trapping_rain_water;

use stack::evaluate_reverse_polish_notation;
use stack::min_stack;
use stack::valid_parenthesis;

use binary_search::koko_eating_bananas;
use binary_search::search_a_2d_matrix;
use binary_search::time_based_key_value_store;

fn main() {
    // let nums = vec![2, 7, 11, 15];
    // let target = 9;
    // let result = two_sum::two_sum(&nums, target);
    // println!("{:?}", result);
    //
    //
    // let nums = vec![1, 2, 3, 4];
    // let result = contains_duplicate::contains_duplicate(&nums);
    // assert!(!result);
    // println!("{:?}", result);
    //
    //
    // let nums = vec![1, 1, 1, 2, 2, 3];
    // let k = 2;
    // let result = top_k_freq::top_k_frequent(nums, k);
    // println!("{:?}", result);
    //
    // let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    // let result = trapping_rain_water::Solution::trap(height);
    // println!("{:?}", result);
    // assert_eq!(result, 6);
    //

    // let input: String = String::from("{{{()}()}}");
    // let result = valid_parenthesis::Solution::is_valid(input);
    // println!("{:?}", result);
}
