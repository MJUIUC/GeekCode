# Arrays & Hashing Patterns

## The Core Idea

Arrays and hashing problems don't have one unifying algorithm like binary search. Instead, they're a **toolkit** of recurring tricks. The common thread: **if you're doing repeated lookups or comparisons, throw it in a HashMap or HashSet.**

Most brute force solutions for these problems are O(n²) — nested loops checking every pair or combination. The hash map/set brings that down to O(n) by trading space for time.

---

## The Patterns

### 1. Hash Map for O(1) Lookup

**When to use:** You're searching for a complement, pair, or matching value inside a loop.

**The trick:** Instead of scanning the whole array for each element, store what you've seen and look it up instantly.

```rust
// Brute force O(n²) — check every pair
for i in 0..nums.len() {
    for j in (i+1)..nums.len() {
        if nums[i] + nums[j] == target { ... }
    }
}

// Hash map O(n) — look up the complement
let mut map = HashMap::new();
for (i, &num) in nums.iter().enumerate() {
    let complement = target - num;
    if let Some(&j) = map.get(&complement) {
        return Some((j, i));
    }
    map.insert(num, i);
}
```

**Key insight:** "Do I need to find something that pairs with the current element?" → HashMap.

**Examples:**
- Two Sum — store seen values, look up complements

---

### 2. Frequency Counting

**When to use:** You need to know how many times each element appears.

**The trick:** Build a frequency map, then use the counts to answer the question.

```rust
let mut freq = HashMap::new();
for num in nums {
    *freq.entry(num).or_insert(0) += 1;
}
```

The `entry` API is idiomatic Rust for this:
- `.entry(key)` — look up the key
- `.or_insert(0)` — if missing, insert 0 and return a mutable reference
- `*` and `+= 1` — dereference and increment

**Examples:**
- Valid Anagram — count chars in both strings, compare counts
- Top K Frequent — count frequencies, sort by count, take top k
- Group Anagrams — use sorted characters as a key to group by frequency signature

---

### 3. Hash Set for Existence Checks

**When to use:** You only care about "have I seen this?" — not "how many times?"

```rust
let set: HashSet<i32> = nums.iter().cloned().collect();

// O(1) lookup
if set.contains(&value) { ... }
```

**How to pick HashMap vs HashSet:**
- Need to associate a value (index, count, etc.)? → **HashMap**
- Just need "is it there?" → **HashSet**

**Examples:**
- Contains Duplicate — if set size < array size, there are duplicates
- Longest Consecutive Sequence — set for O(1) lookup, start counting only from sequence beginnings
- Valid Sudoku — set per row, column, and sub-box to check for duplicate digits

---

### 4. Canonical Form / Normalized Key

**When to use:** You need to group things that are "equivalent" in some way.

**The trick:** Transform each element into a standard form so equivalent items produce the same key.

```rust
// Group anagrams: "eat", "tea", "ate" all sort to "aet"
let mut key: Vec<char> = word.chars().collect();
key.sort();
let key: String = key.into_iter().collect();

map.entry(key).or_default().push(word);
```

**Ask yourself:** "What makes two elements 'the same' for this problem?" Then build a key from that.

**Examples:**
- Group Anagrams — sorted characters as the key
- Could also use: character frequency as a tuple, hash of counts, etc.

---

### 5. Prefix / Running Computation

**When to use:** Each answer depends on all elements to the left and/or right.

**The trick:** Pre-compute cumulative results in one or two passes instead of recalculating from scratch for each position.

```rust
// Product of Array Except Self
// Pass 1: left products
let mut left = 1;
for i in 0..n {
    ans[i] = left;
    left *= nums[i];
}

// Pass 2: right products
let mut right = 1;
for i in (0..n).rev() {
    ans[i] *= right;
    right *= nums[i];
}
```

**Key insight:** If you find yourself writing "for each element, loop through all the others," ask: "can I precompute the answer from each direction?"

**Examples:**
- Product Except Self — left pass × right pass
- Also common: prefix sums, running max/min

---

### 6. Sequence Detection with Smart Starting Points

**When to use:** You need to find sequences or streaks, but checking from every element is too slow.

**The trick:** Only start counting from elements that are the **beginning** of a sequence.

```rust
for &num in &num_set {
    // Only start if this is the beginning of a sequence
    if !num_set.contains(&(num - 1)) {
        let mut len = 1;
        let mut val = num + 1;
        while num_set.contains(&val) {
            len += 1;
            val += 1;
        }
        longest = longest.max(len);
    }
}
```

**Key insight:** Without the `if !set.contains(&(num - 1))` check, you'd recount the same sequence from every element in it. The check ensures each sequence is counted exactly once.

**Examples:**
- Longest Consecutive Sequence

---

## Quick Reference

| Problem Shape | Pattern | Data Structure |
|---|---|---|
| "Find a pair that satisfies X" | O(1) Lookup | HashMap |
| "How many of each?" | Frequency Count | HashMap |
| "Any duplicates?" / "Seen before?" | Existence Check | HashSet |
| "Group equivalent items" | Canonical Key | HashMap<Key, Vec> |
| "Result depends on all other elements" | Prefix/Running Computation | Array + two passes |
| "Find longest streak/sequence" | Smart Starting Points | HashSet |

---

## How to Recognize These Problems

Look for these clues in the problem statement:

| Clue in the problem | Likely pattern |
|---|---|
| "Find a pair/two elements that..." | Hash Map for O(1) lookup |
| "Count", "frequency", "how many times" | Frequency counting |
| "Duplicates", "unique", "seen before" | HashSet |
| "Group", "categorize", "anagram" | Canonical form / normalized key |
| "Product/sum of all except current" | Prefix / running computation |
| "Consecutive", "sequence", "streak" | Smart starting points + HashSet |
| "Subarray sum equals k" | Prefix sum + HashMap |
| O(n) required but brute force is O(n²) | Almost always HashMap or HashSet |

### Red flags that you need hashing

- You're writing a nested loop where the inner loop searches for something → HashMap
- You're checking "does X exist in the array?" repeatedly → HashSet
- You're comparing elements pairwise → frequency count or HashMap
- The constraint says n ≤ 10⁵ and you need better than O(n²) → hash-based approach

---

## Common Rust Idioms for These Problems

### Building a HashMap from a loop
```rust
let mut map = HashMap::new();
for item in items {
    *map.entry(item).or_insert(0) += 1;
}
```

### Building a HashSet from a Vec
```rust
let set: HashSet<i32> = nums.into_iter().collect();
```

### Grouping items by key
```rust
let mut groups: HashMap<Key, Vec<Value>> = HashMap::new();
for item in items {
    let key = compute_key(&item);
    groups.entry(key).or_default().push(item);
}
```

### Converting grouped HashMap to Vec<Vec>
```rust
groups.into_values().collect()
```
