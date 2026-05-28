struct Solution {}
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix[0].len();
        let n = matrix.len();

        let mut right = m * n;
        let mut left = 0;

        while left < right {
            let mid = (left + right) / 2;
            let row = mid / m;
            let col = mid % m;

            if matrix[row][col] == target {
                return true;
            } else if matrix[row][col] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn target_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix, 3));
    }

    #[test]
    fn target_not_found() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(!Solution::search_matrix(matrix, 13));
    }

    #[test]
    fn target_is_first_element() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix, 1));
    }

    #[test]
    fn target_is_last_element() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        assert!(Solution::search_matrix(matrix, 60));
    }

    #[test]
    fn single_element_found() {
        let matrix = vec![vec![5]];
        assert!(Solution::search_matrix(matrix, 5));
    }

    #[test]
    fn single_element_not_found() {
        let matrix = vec![vec![5]];
        assert!(!Solution::search_matrix(matrix, 3));
    }

    #[test]
    fn single_row() {
        let matrix = vec![vec![1, 3, 5, 7, 9]];
        assert!(Solution::search_matrix(matrix, 7));
    }

    #[test]
    fn single_column() {
        let matrix = vec![vec![1], vec![3], vec![5]];
        assert!(Solution::search_matrix(matrix, 3));
    }
}
