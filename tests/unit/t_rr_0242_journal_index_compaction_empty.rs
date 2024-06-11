//! Integration test for `RR-0242` (empty).
//! Journal index compaction integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0242_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0242_journal_index_compaction::evaluate(&[]).is_err(), "RR-0242: empty input must fail for Journal index compaction integrate validator v27");
}
