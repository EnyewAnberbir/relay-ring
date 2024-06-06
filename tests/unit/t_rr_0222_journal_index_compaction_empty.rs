//! Integration test for `RR-0222` (empty).
//! Journal index compaction integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0222_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0222_journal_index_compaction::evaluate(&[]).is_err(), "RR-0222: empty input must fail for Journal index compaction integrate validator v7");
}
