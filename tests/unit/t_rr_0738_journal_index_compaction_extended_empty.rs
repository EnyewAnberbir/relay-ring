//! Integration test for `RR-0738` (empty).
//! Extended: Journal index compaction wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0738_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0738_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0738: empty input must fail for Extended: Journal index compaction wire planner v23");
}
