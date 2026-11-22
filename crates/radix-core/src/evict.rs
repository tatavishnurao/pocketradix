//! LRU eviction of unlocked leaves.

use crate::tree::RadixTree;

/// Evicts unlocked tokens in LRU order until about `_n_tokens` are freed.
/// Returns the number of tokens actually evicted.
pub fn evict(_tree: &mut RadixTree, _n_tokens: usize) -> usize {
    todo!()
}
