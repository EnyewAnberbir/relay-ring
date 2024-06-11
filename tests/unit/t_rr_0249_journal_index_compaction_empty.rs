//! Integration test for `RR-0249` (empty).
//! Journal index compaction optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0249_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0249_journal_index_compaction::evaluate(&[]).is_err(), "RR-0249: empty input must fail for Journal index compaction optimize registry v34");
}
