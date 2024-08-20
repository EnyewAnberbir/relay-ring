//! Integration test for `RR-0741` (empty).
//! Extended: Journal index compaction export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0741_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0741_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0741: empty input must fail for Extended: Journal index compaction export adapter v26");
}
