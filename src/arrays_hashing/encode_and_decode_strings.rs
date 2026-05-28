struct Codec {
    delimiter: String,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self {
            delimiter: "⌘".to_string(),
        }
    }

    fn encode(&self, strs: Vec<String>) -> String {
        let mut out = String::new();
        for s in strs {
            out.push_str(&s);
            out.push_str(&self.delimiter);
        }
        return out;
    }

    fn decode(&self, s: String) -> Vec<String> {
        let mut out = vec![];
        for c in s.split(&self.delimiter) {
            out.push(c.to_string());
        }
        out.pop();
        return out;
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_and_decode_basic() {
        let codec = Codec::new();
        let input = vec!["hello".to_string(), "world".to_string()];
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded);
        assert_eq!(decoded, input);
    }

    #[test]
    fn encode_and_decode_empty_list() {
        let codec = Codec::new();
        let input: Vec<String> = vec![];
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded);
        assert_eq!(decoded, input);
    }

    #[test]
    fn encode_and_decode_empty_strings() {
        let codec = Codec::new();
        let input = vec!["".to_string(), "".to_string()];
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded);
        assert_eq!(decoded, input);
    }

    #[test]
    fn encode_and_decode_single_element() {
        let codec = Codec::new();
        let input = vec!["alone".to_string()];
        let encoded = codec.encode(input.clone());
        let decoded = codec.decode(encoded);
        assert_eq!(decoded, input);
    }
}
