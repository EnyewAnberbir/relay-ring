//! Integration test for `RR-0227` (empty).
//! Journal index compaction harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0227_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0227_journal_index_compaction::evaluate(&[]).is_err(), "RR-0227: empty input must fail for Journal index compaction harden index v12");
}
