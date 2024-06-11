//! Integration test for `RR-0245` (empty).
//! Journal index compaction implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0245_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0245_journal_index_compaction::evaluate(&[]).is_err(), "RR-0245: empty input must fail for Journal index compaction implement pipeline v30");
}
