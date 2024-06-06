//! Integration test for `RR-0224` (empty).
//! Journal index compaction benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0224_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0224_journal_index_compaction::evaluate(&[]).is_err(), "RR-0224: empty input must fail for Journal index compaction benchmark reporter v9");
}
