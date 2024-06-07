//! Integration test for `RR-0226` (empty).
//! Journal index compaction extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0226_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0226_journal_index_compaction::evaluate(&[]).is_err(), "RR-0226: empty input must fail for Journal index compaction extend codec v11");
}
