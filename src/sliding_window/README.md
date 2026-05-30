# Sliding Window Patterns

## The Core Idea

Sliding window avoids recomputing results from scratch for overlapping subsets of data. Instead of examining every possible subarray/substring independently, you **maintain a window** that expands and contracts, updating state incrementally — add one element on the right, remove one on the left.

The key mental model: **a caterpillar**. The head (right pointer) reaches forward to explore. The tail (left pointer) catches up when the body gets too stretched. It only moves forward — neither pointer ever goes backward.

---

## The Two Types

### 1. Fixed-Size Window

The window is always the same length. Right pointer advances by one, left pointer advances by one.

**When to use:** The problem specifies a window/substring of length k.

```rust
for j in 0..len {
    // add right element to state
    state.add(chars[j]);

    // window reached target size — shrink left
    if j >= k {
        state.remove(chars[j - k]);
    }

    // check/record result
    if j >= k - 1 {
        // window is full, check condition
    }
}
```

**Examples:**
- Permutation in String — fixed window of size `s1.len()`

---

### 2. Variable-Size Window

The window grows and shrinks based on a condition. Right pointer always advances. Left pointer only advances when the window becomes invalid.

**When to use:** The problem asks for the longest/shortest substring satisfying some condition.

```rust
let mut left = 0;
for right in 0..len {
    // expand: add right element to state
    state.add(chars[right]);

    // shrink: while window is invalid
    while invalid_condition {
        state.remove(chars[left]);
        left += 1;
    }

    // record result
    longest = longest.max(right - left + 1);
}
```

**Examples:**
- Longest Repeating Character Replacement — variable window, invalid when `window_len - max_freq > k`
- Longest Substring Without Repeating Characters — variable window, invalid when duplicates exist

---

## Key Principle: Slide, Don't Rebuild

The most important optimization in sliding window: **update state incrementally**.

```
Rebuild (slow):           Slide (fast):
┌─────────┐              ┌─────────┐
│ a b c d │ clear        │ a b c d │ remove a
└─────────┘              └─────────┘
  ┌─────────┐              ┌─────────┐
  │ b c d e │ rebuild      │ b c d e │ add e
  └─────────┘              └─────────┘

Rebuild: m operations     Slide: 2 operations
per window position       per window position
```

This applies to any state you track — frequency maps, sums, counts, sets.

### With a HashMap

```rust
// Add right element
*window_map.entry(chars[right]).or_insert(0) += 1;

// Remove left element (when shrinking)
if let Some(count) = window_map.get_mut(&chars[left]) {
    *count -= 1;
    if *count == 0 {
        window_map.remove(&chars[left]);  // remove zeros for clean == comparison
    }
}
```

### With a fixed array (faster)

```rust
// For lowercase letters — 26 slots, no hashing overhead
let mut freq = [0i32; 26];

// Add
freq[(chars[right] as u8 - b'a') as usize] += 1;

// Remove
freq[(chars[left] as u8 - b'a') as usize] -= 1;

// Compare — no need to remove zeros, [0,1,0,...] == [0,1,0,...] works
```

---

## Tracking Window Validity

### Frequency Map Comparison

Compare the window's frequency map to a target map. O(m) per check where m = unique characters.

```rust
if window_map == target_map {
    // window is a permutation / anagram
}
```

**Gotcha:** Remove entries with count 0 from HashMaps, otherwise `{'a': 0}` ≠ `{}`.

### Matches Counter (O(1) per check)

Instead of comparing maps/arrays each time, track how many character slots already match. Update the counter when adding/removing characters:

```rust
// When a character's count changes, check if it now matches or unmatches
if freq_window[idx] == freq_target[idx] { matches += 1; }
if freq_window[idx] == freq_target[idx] + 1 { matches -= 1; } // was matching, now isn't

if matches == 26 { /* valid */ }
```

---

## The Validity Condition

Each problem has a different condition for when the window is valid or invalid:

| Problem | Invalid when |
|---|---|
| Longest Repeating Character Replacement | `window_len - max_freq > k` (too many replacements) |
| Permutation in String | `window_map != target_map` (not an anagram) |
| Longest Substring Without Repeats | Duplicate character in window |
| Minimum Window Substring | Not all target characters are present |

---

## Quick Reference

| Situation | Action |
|---|---|
| Always advance right | `right += 1` or `for right in 0..len` |
| Window too big / invalid | Remove `chars[left]` from state, `left += 1` |
| Window valid | Record result (`max`, `min`, `true`, etc.) |
| Fixed-size window | Shrink when `right - left + 1 > k` |
| Variable-size window | Shrink while condition is invalid |

---

## How to Recognize These Problems

Look for these clues in the problem statement:

| Clue in the problem | Likely pattern |
|---|---|
| "Longest/shortest substring/subarray" | Variable-size window |
| "Substring containing all characters of..." | Variable window + frequency map |
| "Permutation/anagram in a string" | Fixed-size window + frequency comparison |
| "Contiguous subarray of size k" | Fixed-size window |
| "Maximum sum of subarray of length k" | Fixed-size window + running sum |
| "At most k distinct characters" | Variable window + HashMap tracking distinct count |
| "Minimum window containing..." | Variable window, shrink when valid (not invalid) |

### Red flags that you need sliding window

- The problem asks about contiguous subarrays or substrings (not subsequences)
- You need the longest/shortest something that satisfies a condition
- Brute force would be O(n²) checking every start/end pair
- The condition can be checked/maintained incrementally as you add/remove elements
- The problem involves "at most k" replacements, removals, or distinct elements
- You find yourself clearing and rebuilding state — that's the signal to slide instead

---

## Common Pitfalls

### Forgetting to remove zeros from HashMap
```rust
// Without removal: {'a': 0, 'b': 1} != {'b': 1}
// The == comparison fails even though they're logically equal
if *count == 0 {
    window_map.remove(&key);
}
```

### Rebuilding instead of sliding
```rust
// Slow — O(n * m)
window_map.clear();
j = i;

// Fast — O(n)
window_map.remove_or_decrement(chars[i]);
i += 1;
```

### Off-by-one with window size
```rust
// Window from i to j (inclusive) has length:
let len = j - i + 1;  // not j - i
```

### Using `while` vs `if` for shrinking
- **Fixed-size window:** `if` — shrink exactly once per step
- **Variable-size window:** `while` — may need to shrink multiple times
- **Exception:** If the window can only become invalid by one element, `if` works for variable too (like Longest Repeating Character Replacement)
