//! Integration test for `RR-0228` (basic).
//! Journal index compaction wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0228_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe7, 0xe9];
    let first = relayring::capabilities::rr_0228_journal_index_compaction::evaluate(fixture).expect("RR-0228: Journal index compaction wire planner v13");
    let second = relayring::capabilities::rr_0228_journal_index_compaction::evaluate(fixture).expect("RR-0228: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0228: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0228: stats visits every byte");
}
