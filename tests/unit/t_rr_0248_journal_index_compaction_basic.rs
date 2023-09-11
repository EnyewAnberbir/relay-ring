//! Integration test for `RR-0248` (basic).
//! Journal index compaction wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0248_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfb, 0xfd];
    let first = relayring::capabilities::rr_0248_journal_index_compaction::evaluate(fixture).expect("RR-0248: Journal index compaction wire planner v33");
    let second = relayring::capabilities::rr_0248_journal_index_compaction::evaluate(fixture).expect("RR-0248: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0248: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0248: stats visits every byte");
}
