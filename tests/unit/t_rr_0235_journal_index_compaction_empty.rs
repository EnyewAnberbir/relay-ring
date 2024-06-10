//! Integration test for `RR-0235` (empty).
//! Journal index compaction implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0235_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0235_journal_index_compaction::evaluate(&[]).is_err(), "RR-0235: empty input must fail for Journal index compaction implement pipeline v20");
}
