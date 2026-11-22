//! Dumb reference model: stores every sequence and answers by linear scan.

/// Cached token sequences, compared by brute force.
#[derive(Debug, Default)]
pub struct Oracle {
    seqs: Vec<Vec<u32>>,
}

impl Oracle {
    /// Creates an empty oracle.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a cached sequence.
    pub fn insert(&mut self, seq: Vec<u32>) {
        self.seqs.push(seq);
    }

    /// Length of the longest prefix of `query` shared with any cached sequence.
    pub fn longest_common_prefix(&self, query: &[u32]) -> usize {
        self.seqs
            .iter()
            .map(|s| s.iter().zip(query).take_while(|(a, b)| a == b).count())
            .max()
            .unwrap_or(0)
    }
}

#[test]
fn empty_oracle_matches_nothing() {
    let o = Oracle::new();
    assert_eq!(o.longest_common_prefix(&[1, 2, 3]), 0);
}

#[test]
fn exact_prefix_is_full_length() {
    let mut o = Oracle::new();
    o.insert(vec![1, 2, 3, 4]);
    assert_eq!(o.longest_common_prefix(&[1, 2, 3]), 3);
}

#[test]
fn picks_longest_across_sequences() {
    let mut o = Oracle::new();
    o.insert(vec![1, 2, 9]);
    o.insert(vec![1, 2, 3, 7]);
    o.insert(vec![5]);
    assert_eq!(o.longest_common_prefix(&[1, 2, 3, 8]), 3);
}
