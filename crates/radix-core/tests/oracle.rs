//! T0 gate tests for the reference oracle.

mod common;
use common::*;

#[test]
fn empty_cache_matches_nothing() {
    let o = Oracle::new();
    assert_eq!(o.longest_match(&[1, 2, 3]), 0);
}

#[test]
fn empty_query_matches_nothing() {
    let mut o = Oracle::new();
    o.insert(&[1, 2, 3]);
    assert_eq!(o.longest_match(&[]), 0);
}

#[test]
fn exact_repeat() {
    let mut o = Oracle::new();
    o.insert(&[1, 2, 3]);
    assert_eq!(o.longest_match(&[1, 2, 3]), 3);
}

#[test]
fn query_shorter_than_cached() {
    let mut o = Oracle::new();
    o.insert(&[1, 2, 3]);
    assert_eq!(o.longest_match(&[1, 2]), 2);
}

#[test]
fn query_longer_than_cached() {
    let mut o = Oracle::new();
    o.insert(&[1, 2]);
    assert_eq!(o.longest_match(&[1, 2, 3, 4]), 2);
}

#[test]
fn diverges_midway() {
    let mut o = Oracle::new();
    o.insert(&[1, 2, 3, 4]);
    assert_eq!(o.longest_match(&[1, 2, 9, 9]), 2);
}

#[test]
fn best_of_many() {
    let mut o = Oracle::new();
    o.insert(&[1, 9]);
    o.insert(&[1, 2, 7]);
    o.insert(&[5]);
    assert_eq!(o.longest_match(&[1, 2, 3]), 2);
}

#[test]
fn prefix_len_symmetric() {
    let pairs: [(&[u32], &[u32]); 4] = [
        (&[], &[1, 2]),
        (&[1, 2, 3], &[1, 2]),
        (&[1, 2, 3], &[1, 9, 3]),
        (&[4, 5], &[4, 5]),
    ];
    for (a, b) in pairs {
        assert_eq!(common_prefix_len(a, b), common_prefix_len(b, a));
    }
}
