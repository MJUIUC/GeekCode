struct Solution {}

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut eval_stack: Vec<String> = vec![];

        for token in tokens {
            match token.as_str() {
                "+" | "-" | "*" | "/" => {
                    let operand_b: i32 = eval_stack.pop().unwrap().parse().unwrap();
                    let operand_a: i32 = eval_stack.pop().unwrap().parse().unwrap();

                    let result: i32 = match token.as_str() {
                        "+" => operand_a + operand_b,
                        "-" => operand_a - operand_b,
                        "*" => operand_a * operand_b,
                        "/" => operand_a / operand_b,
                        _ => unreachable!(), // becase we're in a match case where the token will be one of these
                    };
                    eval_stack.push(result.to_string())
                }
                _ => eval_stack.push(token),
            }
        }

        eval_stack.pop().unwrap().parse::<i32>().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_tokens(strs: Vec<&str>) -> Vec<String> {
        strs.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn basic_addition() {
        let tokens = to_tokens(vec!["2", "1", "+"]);
        assert_eq!(Solution::eval_rpn(tokens), 3);
    }

    #[test]
    fn basic_subtraction() {
        let tokens = to_tokens(vec!["5", "3", "-"]);
        assert_eq!(Solution::eval_rpn(tokens), 2);
    }

    #[test]
    fn multiple_operations() {
        // (2 + 1) * 3 = 9
        let tokens = to_tokens(vec!["2", "1", "+", "3", "*"]);
        assert_eq!(Solution::eval_rpn(tokens), 9);
    }

    #[test]
    fn division_truncates_toward_zero() {
        let tokens = to_tokens(vec!["7", "2", "/"]);
        assert_eq!(Solution::eval_rpn(tokens), 3);
    }

    #[test]
    fn negative_numbers() {
        let tokens = to_tokens(vec!["4", "13", "5", "/", "+"]);
        assert_eq!(Solution::eval_rpn(tokens), 6);
    }

    #[test]
    fn complex_expression() {
        // ((10 * (6 / -132 + 9)) + 17) + 5
        let tokens = to_tokens(vec![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
        ]);
        assert_eq!(Solution::eval_rpn(tokens), 22);
    }

    #[test]
    fn single_number() {
        let tokens = to_tokens(vec!["42"]);
        assert_eq!(Solution::eval_rpn(tokens), 42);
    }
}
