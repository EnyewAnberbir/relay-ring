//! Integration test for `RR-0729` (empty).
//! Extended: Journal index compaction optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0729_journal_index_compaction_extended_empty() {
    assert!(relayring::capabilities::rr_0729_journal_index_compaction_extended::evaluate(&[]).is_err(), "RR-0729: empty input must fail for Extended: Journal index compaction optimize registry v14");
}
