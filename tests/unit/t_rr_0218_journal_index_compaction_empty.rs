//! Integration test for `RR-0218` (empty).
//! Journal index compaction wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0218_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0218_journal_index_compaction::evaluate(&[]).is_err(), "RR-0218: empty input must fail for Journal index compaction wire planner v3");
}
