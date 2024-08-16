//! Integration test for `RR-0718` (empty).
//! Extended: Journal index compaction wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0718_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0718_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0718: empty input must fail for Extended: Journal index compaction wire planner v3");
}
