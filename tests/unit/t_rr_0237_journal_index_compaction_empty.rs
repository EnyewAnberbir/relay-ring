//! Integration test for `RR-0237` (empty).
//! Journal index compaction harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0237_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0237_journal_index_compaction::evaluate(&[]).is_err(), "RR-0237: empty input must fail for Journal index compaction harden index v22");
}
