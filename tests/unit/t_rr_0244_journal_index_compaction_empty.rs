//! Integration test for `RR-0244` (empty).
//! Journal index compaction benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0244_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0244_journal_index_compaction::evaluate(&[]).is_err(), "RR-0244: empty input must fail for Journal index compaction benchmark reporter v29");
}
