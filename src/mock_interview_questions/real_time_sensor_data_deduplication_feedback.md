# Mock Interview Feedback: Real-Time Sensor Data Deduplication

**Position:** Senior Software Engineer, Backend (Waymo-style)
**Date:** 2026-06-26
**Language:** Rust
**Time Spent:** ~60 minutes (with follow-up discussion)

---

## Overall Grade: **B- / Lean No Hire → Hire with reservations**

At most top-tier companies, this would land in the "mixed signal" zone — strong in some areas, needs work in others. Not a clear pass at senior level, but not a fail either. With another month of focused practice, this becomes a solid hire.

---

## Rubric Breakdown

### 1. Problem Solving & Approach (B-)

| Strength | Gap |
|----------|-----|
| Correctly identified HashMap as the core data structure | Initially over-engineered with a global windowing/chunking mental model |
| Got to a working solution | Needed nudges to simplify to per-ID tracking |
| Good instinct to think about the bigger system | Spent time on system design thinking before solving the core algorithm |

**Note:** The initial instinct to think about MQTT, S3, SLAs, and infrastructure is *valuable* in a system design round — but in a coding round, it cost you time. Practice recognizing which round you're in and adjusting.

### 2. Code Correctness (B)

| Strength | Gap |
|----------|-----|
| Final solution passes all test cases | Had a bug inserting old timestamp instead of new on window expiry |
| Logic structure was sound from the start | Didn't catch the bug via dry-run before executing |
| Test was well-structured with fixtures | Used wrong window value (7000 vs 5000), causing confusion |

**The timestamp bug** was a one-token mistake (`event_entry_timestamp` vs `event.timestamp`). Small, but in an interview it signals you aren't tracing through your code carefully enough.

### 3. Communication & Driving (C+)

This is the biggest gap for senior level.

| Senior Expectation | What Happened |
|--------------------|---------------|
| Candidate drives the approach, interviewer observes | Needed prompting to move from system thinking to algorithm |
| Candidate identifies own bugs via dry-run | Needed external hint to find the timestamp bug |
| Candidate proposes data structures for optimization | Proposed heap when prompted, but didn't arrive there independently |
| Candidate names patterns (e.g., "lazy deletion") | Didn't know the term — this is OK, but naming patterns shows depth |

**To improve:** Practice talking through your approach out loud *before* writing code. "Here's my plan: HashMap keyed by ID, value is the last-accepted timestamp. On each call, check if the ID exists and whether the window has expired. Let me trace through the examples..." — this kind of narration is what senior candidates do.

### 4. Follow-up Discussion (A-)

This is where you showed the most senior-level signal.

| Strength |
|----------|
| Correctly identified unbounded memory as the scaling problem |
| Proposed TTL-based eviction independently |
| Reasoned through amortized cost of different eviction strategies |
| Understood why per-call eviction via min-heap is efficient |
| Implemented `BinaryHeap<Reverse<...>>` correctly in Rust |
| Asked practical questions (system time API) rather than faking knowledge |

This part of the interview would have scored well. You reasoned through tradeoffs like a senior engineer.

### 5. Code Quality (B)

| Strength | Nit |
|----------|-----|
| Clean struct design | `.clone()` on `u64` is unnecessary (it's `Copy`) |
| Derived traits appropriately (`Debug`, `PartialEq`) | Some duplicated code between the if/else branches in `process()` |
| Good test with assertion against fixture | `perform_evictions` has a bug — see below |

---

## Bugs In Final Solution

### 1. Infinite loop in `perform_evictions`

```rust
while let Some(entry) = self.eviction_queue.peek() {
    if oldest_entry_timestamp < (latest_event_time - self.window_delta) {
        self.eviction_queue.pop();
        self.events_map.remove_entry(&oldest_entry_id);
    }
    // ← missing `else { break; }` here!
}
```

If the oldest entry is NOT stale, the loop peeks at it forever. You need a `break` in the else branch.

### 2. Missing lazy deletion check

When you pop from the heap, you should verify the timestamp matches what's in the HashMap. If the event was re-accepted with a newer timestamp, the heap entry is stale and should be skipped, not used to evict the HashMap entry.

### 3. Potential underflow

`latest_event_time - self.window_delta` can underflow if the event timestamp is smaller than the window. Use `.saturating_sub()`.

---

## Study Plan for Improvement

### Week 1–2: Core Algorithm Patterns
- [ ] Practice 3 medium LeetCode problems per day in Rust
- [ ] Focus on: HashMap, BinaryHeap, sliding window, two pointers
- [ ] **After writing each solution, dry-run it with examples BEFORE running**
- [ ] Time yourself — aim for 25 min for medium problems

### Week 3–4: Interview Simulation
- [ ] Practice talking aloud while solving (record yourself)
- [ ] Structure: (1) clarify → (2) approach → (3) trace examples → (4) code → (5) test → (6) optimize
- [ ] Do at least 3 full mock interviews with a timer

### Key Concepts to Review
- [ ] Lazy deletion pattern
- [ ] Amortized analysis (when to use it in conversation)
- [ ] Common dual-structure patterns (HashMap + Heap, HashMap + LinkedList for LRU)
- [ ] Rust-specific: `saturating_sub`, `Entry` API, when `Clone` vs `Copy` applies

### Behavioral Prep
- [ ] Prepare 5–6 STAR stories covering: leadership, conflict, ambiguity, failure, mentorship
- [ ] Practice connecting technical decisions to business impact

---

## Summary

You have solid engineering instincts — the system design thinking, the concern for memory, the reasoning about amortized cost. That's real senior signal. The gap is in **speed and independence on the core algorithm** and **narrating your thought process** as you go. Both are very fixable with practice. You're closer than you think.
