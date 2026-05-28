// Koko loves to eat bananas. There are n piles of bananas, the ith pile has piles[i] bananas. The guards have gone and will come back in h hours.

// Koko can decide her bananas-per-hour eating speed of k. Each hour, she chooses some pile of bananas and eats k bananas from that pile. If the pile has less than k bananas, she eats all of them instead and will not eat any more bananas during this hour.

// Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.

// Return the minimum integer k such that she can eat all the bananas within h hours.
struct Solution {}
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();

        while left < right {
            let k = (left + right) / 2;
            let mut hours = 0;

            for i in 0..piles.len() {
                hours += (piles[i] + k - 1) / k;
            }

            if hours <= h {
                right = k;
            } else {
                left = k + 1;
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }

    #[test]
    fn tight_deadline() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }

    #[test]
    fn relaxed_deadline() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }

    #[test]
    fn single_pile() {
        assert_eq!(Solution::min_eating_speed(vec![10], 5), 2);
    }

    #[test]
    fn one_hour_per_pile() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 4), 11);
    }
}
