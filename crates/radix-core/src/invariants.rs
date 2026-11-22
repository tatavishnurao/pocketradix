//! Structural checks for the tree. Each rule is listed in `docs/invariants.md`.

use crate::tree::RadixTree;

/// A violated invariant, with the rule name and a human-readable detail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvariantError {
    /// Short name of the rule that failed.
    pub rule: &'static str,
    /// What was observed.
    pub detail: String,
}

/// Checks every invariant on `_tree`. Returns the first violation found.
pub fn check(_tree: &RadixTree) -> Result<(), InvariantError> {
    todo!()
}
