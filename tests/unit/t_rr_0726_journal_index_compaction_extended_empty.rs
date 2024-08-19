//! Integration test for `RR-0726` (empty).
//! Extended: Journal index compaction extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0726_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0726_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0726: empty input must fail for Extended: Journal index compaction extend codec v11");
}
