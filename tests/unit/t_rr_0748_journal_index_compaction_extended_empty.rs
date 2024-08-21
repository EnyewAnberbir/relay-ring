//! Integration test for `RR-0748` (empty).
//! Extended: Journal index compaction wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0748_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0748_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0748: empty input must fail for Extended: Journal index compaction wire planner v33");
}
