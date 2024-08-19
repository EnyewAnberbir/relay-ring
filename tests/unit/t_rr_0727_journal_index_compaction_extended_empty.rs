//! Integration test for `RR-0727` (empty).
//! Extended: Journal index compaction harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0727_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0727_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0727: empty input must fail for Extended: Journal index compaction harden index v12");
}
