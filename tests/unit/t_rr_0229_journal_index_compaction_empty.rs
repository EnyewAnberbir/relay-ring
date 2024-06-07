//! Integration test for `RR-0229` (empty).
//! Journal index compaction optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0229_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0229_journal_index_compaction::evaluate(&[]).is_err(), "RR-0229: empty input must fail for Journal index compaction optimize registry v14");
}
