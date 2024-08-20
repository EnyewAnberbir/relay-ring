//! Integration test for `RR-0745` (empty).
//! Extended: Journal index compaction implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0745_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0745_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0745: empty input must fail for Extended: Journal index compaction implement pipeline v30");
}
