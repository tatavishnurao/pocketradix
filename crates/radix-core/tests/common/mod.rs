#![allow(dead_code)]
//! Shared test helpers. The `Oracle` is the brute-force reference model.

/// Cached token sequences, answered by brute force; returns nothing on its own.
pub struct Oracle {
    seqs: Vec<Vec<u32>>,
}

impl Oracle {
    /// Returns an empty oracle.
    pub fn new() -> Self {
        Self { seqs: Vec::new() }
    }

    /// Stores a copy of `seq`; returns nothing.
    pub fn insert(&mut self, seq: &[u32]) {
        self.seqs.push(seq.to_vec());
    }

    /// Returns the longest common-prefix length between `query` and any stored sequence.
    pub fn longest_match(&self, query: &[u32]) -> usize {
        let _ = query;
        // GAP 2: return the largest common-prefix length between query and any stored sequence (0 if none)
        todo!()
    }
}

/// Returns the number of equal leading elements of `a` and `b`.
pub fn common_prefix_len(a: &[u32], b: &[u32]) -> usize {
    let _ = (a, b);
    // GAP 1: count equal leading elements of a and b; stop at the first mismatch or the shorter end
    todo!()
}
