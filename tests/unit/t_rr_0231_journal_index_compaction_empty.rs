//! Integration test for `RR-0231` (empty).
//! Journal index compaction export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0231_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0231_journal_index_compaction::evaluate(&[]).is_err(), "RR-0231: empty input must fail for Journal index compaction export adapter v16");
}
