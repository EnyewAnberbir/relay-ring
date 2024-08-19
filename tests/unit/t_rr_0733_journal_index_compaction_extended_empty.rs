//! Integration test for `RR-0733` (empty).
//! Extended: Journal index compaction refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0733_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0733_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0733: empty input must fail for Extended: Journal index compaction refactor mutator v18");
}
