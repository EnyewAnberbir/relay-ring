//! Integration test for `RR-0731` (empty).
//! Extended: Journal index compaction export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0731_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0731_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0731: empty input must fail for Extended: Journal index compaction export adapter v16");
}
