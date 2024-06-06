//! Integration test for `RR-0220` (empty).
//! Journal index compaction validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0220_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0220_journal_index_compaction::evaluate(&[]).is_err(), "RR-0220: empty input must fail for Journal index compaction validate resolver v5");
}
