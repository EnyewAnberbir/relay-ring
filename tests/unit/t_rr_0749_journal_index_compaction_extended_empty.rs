//! Integration test for `RR-0749` (empty).
//! Extended: Journal index compaction optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0749_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0749_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0749: empty input must fail for Extended: Journal index compaction optimize registry v34");
}
