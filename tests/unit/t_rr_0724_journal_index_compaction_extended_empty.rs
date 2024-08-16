//! Integration test for `RR-0724` (empty).
//! Extended: Journal index compaction benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0724_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0724_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0724: empty input must fail for Extended: Journal index compaction benchmark reporter v9");
}
