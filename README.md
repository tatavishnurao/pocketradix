# pocketradix

A RadixAttention-style prefix cache for LLM serving, starting with a pure-CPU, zero-dependency tree crate.

Milestone 1 in progress: CPU radix tree

## Layout

- `Cargo.toml`, `rust-toolchain.toml`, `rustfmt.toml`: workspace and tooling
- `.github/workflows/ci.yml`: fmt, clippy, test
- `docs/plan.md`, `docs/invariants.md`: plan and tree rules
- `crates/radix-core/src`: `lib`, `node`, `tree`, `lock`, `evict`, `invariants`
- `crates/radix-core/tests`: `oracle` (reference model), `prop_tree` (proptest)

## Plan

See [docs/plan.md](docs/plan.md).

## License

Apache-2.0. See [LICENSE](LICENSE).
