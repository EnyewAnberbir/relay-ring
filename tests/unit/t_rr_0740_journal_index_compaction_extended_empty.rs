//! Integration test for `RR-0740` (empty).
//! Extended: Journal index compaction validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0740_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0740_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0740: empty input must fail for Extended: Journal index compaction validate resolver v25");
}
