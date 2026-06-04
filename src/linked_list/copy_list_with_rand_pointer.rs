// This problem isn't solvable with safe Rust in the traditional sense.
// Each node has a `random` pointer that can reference *any* node in the list,
// including itself — meaning multiple nodes can own or point to the same node.
// Rust's ownership model only allows one owner per value, and its borrow checker
// disallows multiple mutable references to the same data at the same time.
//
// To solve this you'd need one of:
//   - `Rc<RefCell<Node>>` for shared ownership with interior mutability
//   - `unsafe` raw pointers (`*mut Node`), essentially opting out of the borrow checker
//   - An index-based approach using a `Vec`, where "pointers" are just `usize` indices
//
// This is a known pain point in Rust — graph-like structures with shared or cyclic
// references go against the grain of the ownership model by design.
