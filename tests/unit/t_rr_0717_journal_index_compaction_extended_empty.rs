//! Integration test for `RR-0717` (empty).
//! Extended: Journal index compaction harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0717_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0717_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0717: empty input must fail for Extended: Journal index compaction harden index v2");
}
