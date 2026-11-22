# Invariants

Rules the tree must hold after every operation. `invariants::check` enforces them.

- Evictable tokens + protected tokens == total tokens stored in the tree.
  Why: the accounting that decides how much memory can be freed must add up.
- A node with `lock_ref > 0` is never evicted, and neither is any ancestor of it.
  Why: a running request depends on its whole prefix path staying in memory.
- Splitting an edge never changes the token sequence of any path from the root.
  Why: a split is a representation change; it must not change what is cached.
- Every internal node has at least 2 children or is a locked/valid terminal (no unary chains left uncompressed).
  Why: a radix tree is only compact if single-child chains are merged.
- `lock_ref` of a parent >= max `lock_ref` of its children.
  Why: locking a child locks its path, so a parent can never look less pinned than a child.
- Matching a prompt never returns pages for tokens that were not an exact prefix.
  Why: returning KV pages for a non-matching prefix would silently produce wrong output.
