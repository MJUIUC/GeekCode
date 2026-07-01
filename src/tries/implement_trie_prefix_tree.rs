use std::collections::HashMap;

struct Trie {
    word_table: HashMap<char, Trie>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        todo!()
    }

    fn insert(&self, word: String) {
        todo!()
    }

    fn search(&self, word: String) -> bool {
        todo!()
    }

    fn starts_with(&self, prefix: String) -> bool {
        todo!()
    }
}
