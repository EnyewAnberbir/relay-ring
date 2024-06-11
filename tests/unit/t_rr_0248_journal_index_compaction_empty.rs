//! Integration test for `RR-0248` (empty).
//! Journal index compaction wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0248_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0248_journal_index_compaction::evaluate(&[]).is_err(), "RR-0248: empty input must fail for Journal index compaction wire planner v33");
}
