/**
 * SKIPPED: Implementing a linked list cycle in safe Rust is impractical.
 * Cycles require a node to point back to an already-owned node, which
 * violates Rust's single-ownership rule. The closest safe representation
 * would be Rc<RefCell<ListNode>>, but Rc cycles leak memory since the
 * reference count never reaches zero.
 *
 * ALGORITHM: Floyd's Cycle Detection (Tortoise and Hare) - Robert Floyd, 1960s
 *
 * Use two pointers, slow and fast. Each iteration:
 *   - slow moves 1 node forward
 *   - fast moves 2 nodes forward
 *
 * If there is NO cycle: fast hits None and you return false.
 * If there IS a cycle: fast will eventually lap slow and they will
 * land on the same node — something that can only happen inside a cycle.
 *
 * The key detail: fast must skip TWO nodes, not one. If fast only moved
 * one step ahead it would be identical to slow and they'd never meet
 * inside a cycle in a meaningful way.
 *
 * Time complexity: O(n)
 * Space complexity: O(1) — no extra data structures, just two pointers
 */
