//! Integration test for `RR-0230` (empty).
//! Journal index compaction validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0230_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0230_journal_index_compaction::evaluate(&[]).is_err(), "RR-0230: empty input must fail for Journal index compaction validate resolver v15");
}
