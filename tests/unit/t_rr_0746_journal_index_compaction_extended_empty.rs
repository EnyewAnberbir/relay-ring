//! Integration test for `RR-0746` (empty).
//! Extended: Journal index compaction extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0746_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0746_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0746: empty input must fail for Extended: Journal index compaction extend codec v31");
}
