//! Integration test for `RR-0246` (empty).
//! Journal index compaction extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0246_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0246_journal_index_compaction::evaluate(&[]).is_err(), "RR-0246: empty input must fail for Journal index compaction extend codec v31");
}
