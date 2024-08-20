//! Integration test for `RR-0744` (empty).
//! Extended: Journal index compaction benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0744_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0744_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0744: empty input must fail for Extended: Journal index compaction benchmark reporter v29");
}
