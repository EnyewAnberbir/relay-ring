//! Integration test for `RR-0723` (empty).
//! Extended: Journal index compaction refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0723_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0723_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0723: empty input must fail for Extended: Journal index compaction refactor mutator v8");
}
