//! Integration test for `RR-0233` (empty).
//! Journal index compaction refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0233_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0233_journal_index_compaction::evaluate(&[]).is_err(), "RR-0233: empty input must fail for Journal index compaction refactor mutator v18");
}
