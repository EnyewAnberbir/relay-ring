//! Integration test for `RR-0241` (empty).
//! Journal index compaction export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0241_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0241_journal_index_compaction::evaluate(&[]).is_err(), "RR-0241: empty input must fail for Journal index compaction export adapter v26");
}
