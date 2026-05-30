# Binary Search Patterns

## The Core Idea

Binary search isn't just "find a number in a sorted array." It's a general technique for narrowing down a search space by half each step. Every binary search problem boils down to one question:

**When I find a valid mid, which direction do I keep searching?**

---

## The Three Patterns

### 1. Exact Match

**Question:** "Is this specific value here?"

**When to use:** You're looking for one specific target and return "not found" if it doesn't exist.

```rust
while left <= right {
    let mid = (left + right) / 2;

    if nums[mid] == target {
        return mid;              // found it, done
    } else if nums[mid] < target {
        left = mid + 1;
    } else {
        right = mid - 1;
    }
}
return -1;  // not found
```

**Key trait:** You return as soon as you find it. Both pointers skip past `mid`.

**Examples:**
- Search a 2D Matrix — "is target in this matrix?"
- Search in Rotated Sorted Array — "is target in this array?"

---

### 2. Lower Bound (Find First Valid)

**Question:** "What's the **smallest/first** value that satisfies a condition?"

**When to use:** You find a valid answer but think "maybe there's a smaller one."

```rust
while left < right {
    let mid = (left + right) / 2;

    if condition(mid) {
        right = mid;       // mid is valid, but maybe there's an earlier one → keep it
    } else {
        left = mid + 1;    // mid is too small → skip past it
    }
}
return left;  // left == right, converged on the first valid value
```

**Key trait:** When you find a valid `mid`, you set `right = mid` (keep it, search left for something smaller).

**Examples:**
- Koko Eating Bananas — "what's the **smallest** speed that lets her finish?"
- Minimum in Rotated Sorted Array — "what's the **first** point where the array drops?"

---

### 3. Upper Bound (Find Last Valid)

**Question:** "What's the **largest/last** value that satisfies a condition?"

**When to use:** You find a valid answer but think "maybe there's a bigger one."

```rust
while left < right {
    let mid = (left + right) / 2;

    if condition(mid) {
        left = mid + 1;    // mid is valid, but maybe there's a later one → keep searching right
    } else {
        right = mid;       // mid is too big → block here
    }
}
return left - 1;  // left went one past the last valid, so step back
```

**Key trait:** When you find a valid `mid`, you set `left = mid + 1` (search right for something bigger). Answer is `left - 1`.

**Examples:**
- Time-Based Key-Value Store — "what's the **latest** timestamp ≤ target?"

---

## How to Recognize the Pattern

Ask yourself two questions:

### 1. Is there one exact answer, or a range of valid answers?

- **One exact answer** → Exact match
- **Range of valid answers** → Lower or upper bound

### 2. If there's a range, do you want the first or last valid answer?

- **First valid (minimum)** → Lower bound (`right = mid`, answer at `left`)
- **Last valid (maximum)** → Upper bound (`left = mid + 1`, answer at `left - 1`)

### Quick Reference

| Found valid mid. Now what?          | Pattern     | Move         | Answer at  |
|-------------------------------------|-------------|--------------|------------|
| "I'm done"                         | Exact match | return mid   | `mid`      |
| "Maybe there's a smaller one"      | Lower bound | right = mid  | `left`     |
| "Maybe there's a bigger one"       | Upper bound | left = mid+1 | `left - 1` |

---

## Common Pitfalls

### Integer overflow with mid
```rust
// Can overflow if left + right > i32::MAX
let mid = (left + right) / 2;

// Safe version
let mid = left + (right - left) / 2;
```

### Off-by-one with right's initial value
- Exact match: `right = len - 1` (inclusive)
- Lower/upper bound: `right = len` (exclusive) — because `left` might need to reach `len`

### Forgetting to check bounds on upper bound
```rust
// left - 1 can underflow if no valid entry exists
if left > 0 {
    return nums[left - 1];  // last valid
} else {
    // no valid entry
}
```

### Infinite loops
If `left = mid` (not `mid + 1`) and `mid == left`, the loop never progresses. Always use `mid + 1` on one side.

---

## The "Condition" Trick

Many problems don't look like binary search at first. The trick is to reframe them as:

> "Is there a threshold value where everything below fails and everything above succeeds (or vice versa)?"

If yes, binary search on that threshold.

- Koko: "Can she finish at speed k?" → Yes for all k ≥ answer, no below → lower bound
- Capacity to Ship Packages: "Can we ship with capacity c?" → same pattern
- Split Array Largest Sum: "Can we split so max sum ≤ x?" → same pattern

The binary search isn't on the array — it's on the **answer space**.

---

## How to Recognize These Problems

Look for these clues in the problem statement:

| Clue in the problem | Likely pattern |
|---|---|
| "Sorted array" + "find target" | Exact match binary search |
| "Minimum value that satisfies..." | Lower bound |
| "Maximum value that satisfies..." | Upper bound |
| "Rotated sorted array" | Binary search with sorted-half detection |
| "O(log n) required" | Almost always binary search |
| "Minimum speed/capacity/rate to finish in time" | Binary search on the answer space |
| "Find the peak/valley/turning point" | Binary search on slope direction |

### Red flags that you need binary search

- The input is sorted or can be treated as sorted
- You need O(log n) — the problem says so or n ≤ 10⁹ (too big for O(n))
- You're searching for a threshold where a condition flips from false to true
- The brute force is "try every possible value of X" where X has a huge range
- You can answer "is X valid?" efficiently, and validity is monotonic (all valid above/below a point)
