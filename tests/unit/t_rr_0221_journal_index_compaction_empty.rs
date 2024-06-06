//! Integration test for `RR-0221` (empty).
//! Journal index compaction export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0221_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0221_journal_index_compaction::evaluate(&[]).is_err(), "RR-0221: empty input must fail for Journal index compaction export adapter v6");
}
