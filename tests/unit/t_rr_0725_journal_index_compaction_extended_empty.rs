//! Integration test for `RR-0725` (empty).
//! Extended: Journal index compaction implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0725_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0725_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0725: empty input must fail for Extended: Journal index compaction implement pipeline v10");
}
