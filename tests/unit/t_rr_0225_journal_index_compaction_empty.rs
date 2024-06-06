//! Integration test for `RR-0225` (empty).
//! Journal index compaction implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0225_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0225_journal_index_compaction::evaluate(&[]).is_err(), "RR-0225: empty input must fail for Journal index compaction implement pipeline v10");
}
