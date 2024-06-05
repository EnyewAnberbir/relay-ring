//! Integration test for `RR-0216` (empty).
//! Journal index compaction extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0216_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0216_journal_index_compaction::evaluate(&[]).is_err(), "RR-0216: empty input must fail for Journal index compaction extend codec v1");
}
