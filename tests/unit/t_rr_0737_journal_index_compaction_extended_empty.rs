//! Integration test for `RR-0737` (empty).
//! Extended: Journal index compaction harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0737_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0737_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0737: empty input must fail for Extended: Journal index compaction harden index v22");
}
