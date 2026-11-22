//! Radix tree skeleton. Bodies are filled in milestone 1.

use crate::node::{Node, NodeId};

/// Result of a prefix lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixMatch {
    /// Number of leading query tokens that are cached.
    pub matched_tokens: usize,
    /// Pages backing the matched tokens, in order.
    pub pages: Vec<u32>,
}

/// Arena-backed radix tree over `u32` token ids.
#[derive(Debug)]
pub struct RadixTree {
    /// All nodes; `NodeId` indexes into this vector.
    pub nodes: Vec<Node>,
    /// The root node, which has an empty key.
    pub root: NodeId,
}

impl RadixTree {
    /// Creates an empty tree containing only the root.
    pub fn new() -> Self {
        todo!()
    }

    /// Returns the longest cached prefix of `key` and the pages that back it.
    pub fn match_prefix(&self, _key: &[u32]) -> PrefixMatch {
        todo!()
    }

    /// Inserts `key` with its backing `pages`, splitting edges where needed.
    pub fn insert(&mut self, _key: &[u32], _pages: &[u32]) {
        todo!()
    }
}

impl Default for RadixTree {
    fn default() -> Self {
        Self::new()
    }
}
