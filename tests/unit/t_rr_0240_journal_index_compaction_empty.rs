//! Integration test for `RR-0240` (empty).
//! Journal index compaction validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0240_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0240_journal_index_compaction::evaluate(&[]).is_err(), "RR-0240: empty input must fail for Journal index compaction validate resolver v25");
}
