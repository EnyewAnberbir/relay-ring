//! Integration test for `RR-0236` (empty).
//! Journal index compaction extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0236_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0236_journal_index_compaction::evaluate(&[]).is_err(), "RR-0236: empty input must fail for Journal index compaction extend codec v21");
}
