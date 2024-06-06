//! Integration test for `RR-0223` (empty).
//! Journal index compaction refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0223_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0223_journal_index_compaction::evaluate(&[]).is_err(), "RR-0223: empty input must fail for Journal index compaction refactor mutator v8");
}
