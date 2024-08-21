//! Integration test for `RR-0747` (empty).
//! Extended: Journal index compaction harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0747_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0747_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0747: empty input must fail for Extended: Journal index compaction harden index v32");
}
