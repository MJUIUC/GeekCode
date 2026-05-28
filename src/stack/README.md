# Stack Patterns

## The Core Idea

Stacks are useful whenever you need to process things in **LIFO order**, track the "most recent" state, or match pairs. The common thread: you're working through a sequence and need to remember something you'll come back to later — the last thing you pushed is the first thing you'll need.

**When you see a problem that involves nesting, "nearest previous/next" relationships, or undoing the most recent action, reach for a stack.**

---

## The Patterns

### 1. Matching Pairs

**When to use:** You need to match opening and closing elements — brackets, tags, or any symmetric structure.

**The trick:** Push every opening element. When you see a closing element, pop the stack and check that it matches. If the stack is empty at the end, everything matched.

```rust
fn is_valid(input: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let mut map: HashMap<char, char> = HashMap::new();
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
```

**Problems:**
- Valid Parentheses — push opening brackets, pop and compare for closing ones

---

### 2. Monotonic Stack

**When to use:** You need to find the "next greater element," "next smaller element," or the distance to something that breaks a trend.

**The trick:** Maintain a stack where elements are in sorted order (increasing or decreasing). When a new element violates the order, pop and process until the invariant is restored.

> **Key insight:** The stack maintains an invariant (e.g. decreasing temperatures). When a new element violates the invariant, you process elements off the stack until the invariant is restored. This lets you find "next greater/smaller element" relationships in O(n).

```rust
// Daily Temperatures — decreasing stack of (temp, index)
fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let temp_len = temperatures.len();
    let mut mono_stack: Vec<(i32, usize)> = Vec::new();
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
```

```rust
// Car Fleet — stack of arrival times, only push when a new fleet is found
fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut arrival_time_stack: Vec<f64> = vec![];

    let mut sorted_positions: Vec<(i32, i32)> = position.into_iter().zip(speed).collect();
    sorted_positions.sort_by(|a, b| b.0.cmp(&a.0));

    for i in 0..sorted_positions.len() {
        let arrival_time: f64 =
            (target - sorted_positions[i].0) as f64 / sorted_positions[i].1 as f64;
        if arrival_time_stack.is_empty()
            || arrival_time_stack.last().unwrap() < &arrival_time
        {
            arrival_time_stack.push(arrival_time);
        }
    }

    arrival_time_stack.len() as i32
}
```

**Problems:**
- Daily Temperatures — decreasing stack of temperatures; when a warmer day appears, pop and calculate the difference in indices
- Car Fleet — stack of arrival times; only push when a car is slower (forms a new fleet)

---

### 3. Auxiliary / Tracking Stack

**When to use:** You need O(1) access to some running aggregate (minimum, maximum) alongside normal stack operations.

**The trick:** Run a second stack in parallel that tracks additional state. Each entry in the auxiliary stack corresponds to the state at that depth in the main stack. Can be optimized by only pushing to the auxiliary stack when the tracked value actually changes.

```rust
struct MinStack {
    stack: Vec<i32>,
    min_pointer_stack: Vec<usize>,  // stores indices into `stack`
}

impl MinStack {
    fn new() -> Self {
        Self { stack: vec![], min_pointer_stack: vec![] }
    }

    fn push(&mut self, val: i32) {
        let cur_min_pointer: usize = if self.min_pointer_stack.is_empty() {
            self.min_pointer_stack.push(0);
            0
        } else {
            *self.min_pointer_stack.last().unwrap()
        };

        if !self.stack.is_empty() && self.stack[cur_min_pointer] > val {
            self.min_pointer_stack.push(self.stack.len());
        }

        self.stack.push(val);
    }

    fn pop(&mut self) {
        let cur_pointer = self.stack.len() - 1;
        let cur_min_pointer = *self.min_pointer_stack.last().unwrap();

        if cur_pointer == cur_min_pointer {
            self.min_pointer_stack.pop();
        }

        self.stack.pop();
    }

    fn get_min(&self) -> i32 {
        let cur_min_index = *self.min_pointer_stack.last().unwrap();
        self.stack[cur_min_index]
    }
}
```

**Problems:**
- Min Stack — a separate stack records the minimum at each level via index pointers. Optimized to only push on new minimums, so the auxiliary stack can be smaller than the main stack.

---

### 4. Expression Evaluation

**When to use:** You need to evaluate expressions, especially in postfix (RPN) or infix notation with operator precedence.

**The trick:** Push operands onto the stack. When you hit an operator, pop the required operands, compute the result, and push it back. The stack naturally handles operator precedence and nesting.

```rust
fn eval_rpn(tokens: Vec<String>) -> i32 {
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
                    _ => unreachable!(),
                };
                eval_stack.push(result.to_string())
            }
            _ => eval_stack.push(token),
        }
    }

    eval_stack.pop().unwrap().parse::<i32>().unwrap()
}
```

**Problems:**
- Evaluate Reverse Polish Notation — push numbers, pop two when you see `+`, `-`, `*`, `/`

---

## Quick Reference

| Problem shape                                    | Pattern              | Example problem                     |
|--------------------------------------------------|----------------------|-------------------------------------|
| Match opening/closing elements                   | Matching Pairs       | Valid Parentheses                    |
| Find next greater/smaller element                | Monotonic Stack      | Daily Temperatures                  |
| Group elements that "catch up" to each other     | Monotonic Stack      | Car Fleet                           |
| Track running min/max alongside push/pop         | Auxiliary Stack      | Min Stack                           |
| Evaluate postfix or infix expressions            | Expression Evaluation| Evaluate Reverse Polish Notation    |

---

## Common Pitfalls

### Empty stack unwrap panics
```rust
// This panics if the stack is empty
let top = stack.pop().unwrap();

// Guard it
if let Some(top) = stack.pop() {
    // process top
}
```

Always check `stack.is_empty()` or use `if let` / `match` before calling `.unwrap()` on `pop()` or `last()`.

### `last()` vs `pop()`
- `last()` **peeks** — returns `Option<&T>`, the stack is unchanged
- `pop()` **removes** — returns `Option<T>`, the element is gone

Use `last()` when you need to compare against the top without consuming it (e.g. checking the monotonic invariant). Use `pop()` when you're ready to process and discard.

### `usize` underflow
```rust
// If i is 0, this panics in debug or wraps in release
let prev = i - 1;

// Safe alternatives
let prev = i.checked_sub(1);          // returns Option<usize>
let diff = i.abs_diff(j);             // always safe, returns usize
let diff = (i as i32) - (j as i32);   // cast first if you need a signed result
```

Stack index arithmetic often involves subtracting indices. Since Rust uses `usize` (unsigned), subtracting a larger value from a smaller one causes a panic in debug builds or silent wraparound in release. Use `abs_diff()`, `checked_sub()`, or cast to a signed type first.

### Forgetting leftover elements
After processing the input, elements may remain on the stack. Depending on the problem:
- **Matching Pairs:** leftover elements mean unmatched input → return `false`
- **Monotonic Stack:** leftover elements had no "next greater" → their answer stays at the default (usually `0`)
- **Expression Evaluation:** exactly one element should remain → that's your answer
