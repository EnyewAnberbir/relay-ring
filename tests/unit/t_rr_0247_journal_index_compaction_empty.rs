//! Integration test for `RR-0247` (empty).
//! Journal index compaction harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0247_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0247_journal_index_compaction::evaluate(&[]).is_err(), "RR-0247: empty input must fail for Journal index compaction harden index v32");
}
