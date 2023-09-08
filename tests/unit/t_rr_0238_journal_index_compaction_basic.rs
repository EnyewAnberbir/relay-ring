//! Integration test for `RR-0238` (basic).
//! Journal index compaction wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0238_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xf3];
    let first = relayring::capabilities::rr_0238_journal_index_compaction::evaluate(fixture).expect("RR-0238: Journal index compaction wire planner v23");
    let second = relayring::capabilities::rr_0238_journal_index_compaction::evaluate(fixture).expect("RR-0238: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0238: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0238: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
