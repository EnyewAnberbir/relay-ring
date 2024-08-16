//! Integration test for `RR-0719` (empty).
//! Extended: Journal index compaction optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0719_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0719_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0719: empty input must fail for Extended: Journal index compaction optimize registry v4");
}
