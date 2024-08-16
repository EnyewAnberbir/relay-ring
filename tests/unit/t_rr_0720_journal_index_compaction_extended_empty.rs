//! Integration test for `RR-0720` (empty).
//! Extended: Journal index compaction validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0720_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0720_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0720: empty input must fail for Extended: Journal index compaction validate resolver v5");
}
