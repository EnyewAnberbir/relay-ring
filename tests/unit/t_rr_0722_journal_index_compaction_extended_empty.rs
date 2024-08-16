//! Integration test for `RR-0722` (empty).
//! Extended: Journal index compaction integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0722_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0722_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0722: empty input must fail for Extended: Journal index compaction integrate validator v7");
}
