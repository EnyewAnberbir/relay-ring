//! Integration test for `RR-0228` (empty).
//! Journal index compaction wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0228_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0228_journal_index_compaction::evaluate(&[]).is_err(), "RR-0228: empty input must fail for Journal index compaction wire planner v13");
}
