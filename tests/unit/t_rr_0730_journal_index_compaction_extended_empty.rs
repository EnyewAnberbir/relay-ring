//! Integration test for `RR-0730` (empty).
//! Extended: Journal index compaction validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0730_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0730_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0730: empty input must fail for Extended: Journal index compaction validate resolver v15");
}
