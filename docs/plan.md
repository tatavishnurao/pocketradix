# Plan

| Problem | File | Test gate | Milestone |
|---|---|---|---|
| radix tree (match, split, insert) | crates/radix-core/src/tree.rs | proptest vs Oracle, 10,000 cases | 1 |
| locks | lock.rs | no locked node is ever evicted | 1 |
| LRU eviction | evict.rs | invariants hold after every op | 1 |
| KV page pool | crates/kv-pool (not created yet) | pool size matches hand computation | 2 |
| model forward | crates/runner (not created yet) | greedy tokens match Hugging Face for 50 tokens | 3 |
| scheduler + measurement | crates/scheduler, bench/ (not created yet) | tailbench TTFT/ITL on shared-prefix workload | 4-6 |
