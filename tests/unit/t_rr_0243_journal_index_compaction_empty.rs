//! Integration test for `RR-0243` (empty).
//! Journal index compaction refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0243_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0243_journal_index_compaction::evaluate(&[]).is_err(), "RR-0243: empty input must fail for Journal index compaction refactor mutator v28");
}
