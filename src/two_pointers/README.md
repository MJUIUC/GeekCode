# Two Pointer Patterns

## The Core Idea

Two pointers avoid nested loops by maintaining two positions that move based on conditions. Instead of checking every pair (O(n²)), you use structure in the data — sorting, symmetry, or monotonicity — to advance one pointer or the other, bringing the problem down to O(n).

**Every two-pointer problem boils down to one question:**

**Which pointer do I move, and why?**

---

## The Patterns

### 1. Opposite-End Squeeze

**When to use:** You need to compare or combine elements from both sides of a sorted or symmetric structure.

**The trick:** Start `left` at 0 and `right` at the end. Move them inward based on what you find — the structure of the data tells you which pointer to advance.

```rust
let mut left = 0;
let mut right = nums.len() - 1;

while left < right {
    let sum = nums[left] + nums[right];
    if sum == target {
        return vec![left, right];
    } else if sum < target {
        left += 1;     // sum too small → need bigger left
    } else {
        right -= 1;    // sum too large → need smaller right
    }
}
```

**Why it works:** In a sorted array, moving `left` right can only increase the sum, and moving `right` left can only decrease it. You never need to go back.

**Problems that use it:**
- **Valid Palindrome** — compare chars from outside in, return `false` on first mismatch
- **Two Sum II** — move `left` up if sum too small, `right` down if too large
- **Trapping Rain Water** — process the shorter side (see [Tracking Running State](#4-tracking-running-state))

---

### 2. Fix One + Sweep

**When to use:** You need to find triplets (or k-tuples) that satisfy a condition. Fixing one element reduces the problem to a two-pointer subproblem.

**The trick:** Sort the array. Iterate `i` over each element, then run an opposite-end squeeze on the remaining elements. Skip duplicates to avoid repeated work.

```rust
let mut nums_sorted = nums.clone();
nums_sorted.sort();
let mut result = Vec::new();

for i in 0..nums_sorted.len() {
    // skip duplicate fixed elements
    if i > 0 && nums_sorted[i] == nums_sorted[i - 1] {
        continue;
    }

    let mut j = i + 1;
    let mut k = nums_sorted.len() - 1;

    while j < k {
        let sum = nums_sorted[i] + nums_sorted[j] + nums_sorted[k];
        if sum == 0 {
            result.push(vec![nums_sorted[i], nums_sorted[j], nums_sorted[k]]);
            j += 1;
            // skip duplicate left pointers
            while j < k && nums_sorted[j] == nums_sorted[j - 1] {
                j += 1;
            }
        } else if sum < 0 {
            j += 1;
        } else {
            k -= 1;
        }
    }
}
```

**Why it works:** Sorting lets you skip duplicates cheaply and guarantees the inner two-pointer sweep is correct. You go from O(n³) brute force to O(n²).

**Problems that use it:**
- **Three Sum** — fix `i`, then two-pointer sweep on the rest to find pairs that sum to `-nums[i]`

---

### 3. Skip/Filter In Place

**When to use:** Some elements are irrelevant (whitespace, punctuation, etc.) and you want to skip them without pre-processing the input.

**The trick:** Use `continue` or a nested `while` loop to advance past invalid elements before doing the real comparison.

```rust
let bytes = s.as_bytes();
let mut left = 0;
let mut right = bytes.len() - 1;

while left < right {
    if !bytes[left].is_ascii_alphanumeric() {
        left += 1;
        continue;
    }
    if !bytes[right].is_ascii_alphanumeric() {
        right -= 1;
        continue;
    }
    if bytes[left].to_ascii_lowercase() != bytes[right].to_ascii_lowercase() {
        return false;
    }
    left += 1;
    right -= 1;
}
true
```

**Why it works:** You avoid allocating a filtered copy of the input. The pointers still move monotonically inward, so it's still O(n).

**Problems that use it:**
- **Valid Palindrome (bytes version)** — skip non-alphanumeric chars in-place instead of pre-filtering into a `Vec<char>`

---

### 4. Tracking Running State

**When to use:** The answer at each position depends on some accumulated property (max so far, running sum, etc.) that you can maintain as the pointers move.

**The trick:** Carry auxiliary state alongside the pointers. Update it incrementally — don't recompute from scratch each step.

```rust
let mut left = 0;
let mut right = height.len() - 1;
let mut max_left = 0;
let mut max_right = 0;
let mut water = 0;

while left < right {
    if height[left] <= height[right] {
        max_left = max_left.max(height[left]);
        water += max_left - height[left];
        left += 1;
    } else {
        max_right = max_right.max(height[right]);
        water += max_right - height[right];
        right -= 1;
    }
}
```

**The key insight for Trapping Rain Water:** You only need to know that the other side is tall enough — the shorter side is always the bottleneck. If `height[left] <= height[right]`, then the water at `left` is bounded by `max_left`, regardless of what's happening on the right. You know the right side is *at least* as tall as the left, so the left side is the constraint. That's why you always process the shorter side.

**Problems that use it:**
- **Trapping Rain Water** — track `max_left` and `max_right`, process the shorter side each step

---

## Quick Reference

| Problem Shape                                   | Pattern                | Time   |
|-------------------------------------------------|------------------------|--------|
| Compare/combine from both ends of sorted data   | Opposite-End Squeeze   | O(n)   |
| Find pairs in a sorted array                    | Opposite-End Squeeze   | O(n)   |
| Find triplets that satisfy a condition           | Fix One + Sweep        | O(n²)  |
| Skip irrelevant elements during traversal        | Skip/Filter In Place   | O(n)   |
| Answer depends on running max/min/sum            | Tracking Running State | O(n)   |
| "Is this string a palindrome?"                  | Opposite-End Squeeze   | O(n)   |
| "How much water can be trapped?"                | Squeeze + Running State| O(n)   |

---

## Common Pitfalls

### `usize` underflow with the right pointer
`right` is typically `usize` (unsigned). If you write `right -= 1` when `right` is 0, it wraps to `usize::MAX` and you get a panic or an infinite loop.

```rust
// Dangerous: if right is 0, this wraps around
right -= 1;

// Safe: use the loop condition to prevent it
while left < right {   // right is always >= 1 when we enter the body
    // ...
    right -= 1;        // safe because right > left >= 0, so right >= 1
}
```

The `while left < right` guard is your friend — as long as you only decrement `right` inside the loop, it can never underflow.

### Off-by-one: `<` vs `<=`

- **`while left < right`** — use this for most two-pointer problems. The pointers converge and stop when they meet. You never process the same element twice.
- **`while left <= right`** — use this when you need to process the element where `left == right` (e.g., exact-match binary search). Rare in two-pointer problems.

Rule of thumb: if you're comparing `nums[left]` with `nums[right]`, use `<`. If they're the same index, there's nothing to compare.

### Forgetting to skip duplicates in Three Sum
Without duplicate skipping, you get repeated triplets. Always skip when the fixed element or the sweep pointer sees the same value as the previous one.

```rust
// Skip duplicate fixed element
if i > 0 && nums[i] == nums[i - 1] {
    continue;
}

// Skip duplicate left pointer after finding a match
while j < k && nums[j] == nums[j - 1] {
    j += 1;
}
```

### Moving both pointers at once
Don't advance both `left` and `right` in the same step unless you're sure both need to move (like in palindrome checking after a match). In sum-based problems, move only the pointer that fixes the problem.
