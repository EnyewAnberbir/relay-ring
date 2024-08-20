//! Integration test for `RR-0743` (empty).
//! Extended: Journal index compaction refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0743_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0743_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0743: empty input must fail for Extended: Journal index compaction refactor mutator v28");
}
