//! Integration test for `RR-0232` (empty).
//! Journal index compaction integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0232_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0232_journal_index_compaction::evaluate(&[]).is_err(), "RR-0232: empty input must fail for Journal index compaction integrate validator v17");
}
