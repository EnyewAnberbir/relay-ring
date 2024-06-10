//! Integration test for `RR-0239` (empty).
//! Journal index compaction optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0239_journal_index_compaction_empty() {
    assert!(relayring::capabilities::rr_0239_journal_index_compaction::evaluate(&[]).is_err(), "RR-0239: empty input must fail for Journal index compaction optimize registry v24");
}
