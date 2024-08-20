//! Integration test for `RR-0742` (empty).
//! Extended: Journal index compaction integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0742_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0742_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0742: empty input must fail for Extended: Journal index compaction integrate validator v27");
}
