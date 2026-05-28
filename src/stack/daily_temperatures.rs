struct Solution {}
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let temp_len = temperatures.len();
        let mut mono_stack: Vec<(i32, usize)> = Vec::<(i32, usize)>::new();
        let mut answer: Vec<usize> = vec![0; temp_len];

        for i in 0..temp_len {
            while (!mono_stack.is_empty()) && temperatures[i] > mono_stack.last().unwrap().0 {
                let top = mono_stack.pop().unwrap();
                answer[top.1] = i.abs_diff(top.1);
            }
            mono_stack.push((temperatures[i], i));
        }
        answer.iter().map(|x| *x as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_case() {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
        assert_eq!(
            Solution::daily_temperatures(temps),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn all_same() {
        let temps = vec![70, 70, 70, 70];
        assert_eq!(Solution::daily_temperatures(temps), vec![0, 0, 0, 0]);
    }

    #[test]
    fn descending() {
        let temps = vec![80, 70, 60, 50];
        assert_eq!(Solution::daily_temperatures(temps), vec![0, 0, 0, 0]);
    }

    #[test]
    fn ascending() {
        let temps = vec![50, 60, 70, 80];
        assert_eq!(Solution::daily_temperatures(temps), vec![1, 1, 1, 0]);
    }

    #[test]
    fn single_element() {
        let temps = vec![42];
        assert_eq!(Solution::daily_temperatures(temps), vec![0]);
    }

    #[test]
    fn two_elements() {
        let temps = vec![30, 40];
        assert_eq!(Solution::daily_temperatures(temps), vec![1, 0]);
    }
}
