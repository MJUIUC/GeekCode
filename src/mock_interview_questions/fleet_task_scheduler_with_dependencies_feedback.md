# Mock Interview Feedback: Fleet Task Scheduler with Dependencies

**Position:** Senior Software Engineer, Backend (Waymo-style)
**Date:** 2026-06-30
**Language:** Rust
**Time Spent:** ~45 minutes (with one iteration)
**Algorithm:** Topological Sort (Kahn's Algorithm)

---

## Overall Grade: **B+ / Lean Hire**

Clear improvement from Problem 1. You identified the right algorithm family (BFS on a DAG), iterated effectively on feedback, and delivered a clean working solution. With a bit more independence on the first attempt, this is a solid hire.

---

## Rubric Breakdown

### 1. Problem Solving & Approach (B+)

| Strength | Gap |
|----------|-----|
| Immediately recognized this as a graph problem | First attempt used wrong root-finding strategy (min key vs. zero in-degree) |
| Correctly mapped tasks to nodes and dependencies to edges | Cycle detection in attempt 1 was based on "visited at different level" — not reliable |
| Identified BFS as the traversal strategy | Needed a nudge toward in-degree tracking |
| Second attempt nailed it cleanly | — |

**Key takeaway:** You had the right shape in your head from the start. The gap was knowing the specific pattern — Kahn's algorithm. Now that you've implemented it, you'll recognize it instantly next time.

### 2. Code Correctness (A-)

| Strength | Gap |
|----------|-----|
| Final solution passes all test cases | First attempt had a subtle bug with root node selection |
| In-degree decrement logic is correct | Minor: `0 as usize` cast is unnecessary |
| Cycle detection via length check is elegant and correct | — |
| Properly handles the "no dependencies" case | Could add more edge case tests |

The final solution has no bugs. That's a strong signal.

### 3. Code Quality (B+)

| Strength | Nit |
|----------|-----|
| Clear variable names (`n_degree_map`, `dep_map`, `execution_ordering`) | `nodes` vec is allocated but only used for initialization — could iterate `0..num_tasks` directly |
| Good use of Rust idioms (`.entry().and_modify().or_default()`) | HashMap could be replaced with `Vec<usize>` for both maps since keys are `0..num_tasks` — more cache-friendly and idiomatic for indexed nodes |
| Kept attempt 1 for reference | Commented-out `println!` should be removed in final version |

**Optimization note an interviewer might raise:** Since tasks are `0..num_tasks`, you can use `Vec` instead of `HashMap` for both the adjacency list and in-degree tracking. This gives you:
- Better cache locality
- No hashing overhead
- Simpler code

```rust
let mut in_degree = vec![0usize; num_tasks];
let mut adj = vec![vec![]; num_tasks];
```

Not required, but shows systems-level awareness.

### 4. Communication & Driving (B)

| Improvement from Problem 1 | Remaining gap |
|-----------------------------|---------------|
| Moved to code faster, less time on system design tangents | Still needed a nudge to move from "BFS" to "Kahn's with in-degree" |
| Asked targeted questions (e.g., "how do I cast as usize?") | Didn't verbally trace through the examples before running |
| Iterated quickly on feedback | At senior level, interviewer expects you to name the algorithm ("this is topological sort") |

### 5. Testing (B-)

| Strength | Gap |
|----------|-----|
| Test covers the primary happy path | No test for cycle detection (Example 2) |
| Used assertion against fixture | No test for empty dependencies (Example 3) |
| — | No test for disconnected tasks (task with no edges) |
| — | Assertion on exact ordering could be flaky due to HashMap iteration order |

**Recommended additional tests:**

```rust
#[test]
fn cycle_detection() {
    let result = FleetTaskScheduler::schedule_tasks(3, vec![(0, 1), (1, 2), (2, 0)]);
    assert_eq!(result, None);
}

#[test]
fn no_dependencies() {
    let result = FleetTaskScheduler::schedule_tasks(3, vec![]);
    assert!(result.is_some());
    assert_eq!(result.unwrap().len(), 3);
}

#[test]
fn disconnected_task() {
    let result = FleetTaskScheduler::schedule_tasks(3, vec![(2, 1)]);
    assert!(result.is_some());
    let order = result.unwrap();
    assert_eq!(order.len(), 3);
    // task 1 must come before task 2
    assert!(order.iter().position(|&x| x == 1) < order.iter().position(|&x| x == 2));
}
```

---

## Comparison to Problem 1

| Dimension | Problem 1 (Dedup) | Problem 2 (Scheduler) |
|-----------|-------------------|----------------------|
| Time to working solution | ~60 min with nudges | ~45 min with one iteration |
| Bugs in final code | 3 (fixed after review) | 0 |
| Independence | Needed significant guidance | Needed one conceptual nudge |
| Data structure choice | Correct (HashMap) | Correct (HashMap + BFS queue) |
| Testing | Good fixture-based test | Good but missing edge cases |

**Clear upward trend.** You're learning fast within the session.

---

## Study Plan Update

Based on both problems, here's what to focus on:

### Patterns to Drill
- [ ] **Topological sort** — both Kahn's (BFS) and DFS-based approaches
- [ ] **Graph representations** — adjacency list, adjacency matrix, when to use each
- [ ] **BFS/DFS variations** — level-order, cycle detection, shortest path
- [ ] **In-degree / out-degree reasoning** — comes up in scheduling, build systems, dependency resolution

### Interview Habits
- [ ] **Name the algorithm** when you recognize it ("this is topological sort / Kahn's algorithm")
- [ ] **Trace before you run** — walk through 2-3 events/nodes by hand before executing
- [ ] **Write edge case tests early** — cycle, empty input, single node, disconnected components
- [ ] **Use Vec over HashMap** when keys are contiguous integers — shows systems awareness

### Concepts to Review
- [ ] Topological sort (Kahn's BFS + DFS with coloring)
- [ ] Cycle detection in directed vs. undirected graphs
- [ ] DAG properties and applications (build systems, task scheduling, course prerequisites)
- [ ] Difference between BFS and DFS for topological ordering

---

## Summary

Strong improvement from Problem 1 to Problem 2. Your graph intuition is solid — you knew it was BFS on a DAG before writing any code. The main growth area is **pattern recognition speed** (knowing "this is Kahn's" immediately) and **testing discipline** (always write the cycle test for any graph problem). Keep this trajectory and you'll be passing senior coding rounds confidently.
