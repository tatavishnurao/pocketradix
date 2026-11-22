//! Lock reference counting. Rules are in `docs/invariants.md`.

use crate::node::NodeId;
use crate::tree::RadixTree;

/// Increments `lock_ref` on `_node`.
pub fn inc_lock(_tree: &mut RadixTree, _node: NodeId) {
    todo!()
}

/// Decrements `lock_ref` on `_node`.
pub fn dec_lock(_tree: &mut RadixTree, _node: NodeId) {
    todo!()
}
