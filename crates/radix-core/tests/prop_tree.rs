//! Property tests: `RadixTree` must agree with the `Oracle` model.

mod oracle;

use oracle::Oracle;
use proptest::prelude::*;
use radix_core::tree::RadixTree;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

    #[test]
    #[ignore = "milestone 1"]
    fn match_prefix_agrees_with_oracle(
        seqs in prop::collection::vec(prop::collection::vec(0u32..8, 0..16), 0..20),
        query in prop::collection::vec(0u32..8, 0..16),
    ) {
        let mut oracle = Oracle::new();
        let mut tree = RadixTree::new();
        for s in &seqs {
            oracle.insert(s.clone());
            tree.insert(s, s);
        }
        prop_assert_eq!(
            tree.match_prefix(&query).matched_tokens,
            oracle.longest_common_prefix(&query)
        );
    }
}
