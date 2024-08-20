//! Integration test for `RR-0739` (empty).
//! Extended: Journal index compaction optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0739_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0739_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0739: empty input must fail for Extended: Journal index compaction optimize registry v24");
}
