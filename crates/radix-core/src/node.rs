//! Tree node layout. Nodes live in an arena and refer to each other by `NodeId`.

/// Index of a node in `RadixTree::nodes`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub u32);

/// One edge of the radix tree: the token run leading into this node.
#[derive(Debug, Clone)]
pub struct Node {
    /// Token ids on the edge from the parent to this node.
    pub key: Vec<u32>,
    /// Page ids holding the KV cache for `key`, one entry per page.
    pub pages: Vec<u32>,
    /// Child nodes in the arena.
    pub children: Vec<NodeId>,
    /// Parent node, `None` only for the root.
    pub parent: Option<NodeId>,
    /// Number of running requests pinning this node. Locked nodes are not evictable.
    pub lock_ref: u32,
    /// Logical timestamp of the last match or insert, used for LRU ordering.
    pub last_access: u64,
}
