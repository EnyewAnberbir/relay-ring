//! Integration test for `RR-0721` (empty).
//! Extended: Journal index compaction export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0721_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0721_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0721: empty input must fail for Extended: Journal index compaction export adapter v6");
}
