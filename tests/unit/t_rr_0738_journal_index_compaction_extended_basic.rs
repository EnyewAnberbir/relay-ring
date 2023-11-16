//! Integration test for `RR-0738` (basic).
//! Extended: Journal index compaction wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0738_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe9, 0xeb];
    let first = relayring::capabilities::rr_0738_journal_index_compaction_extended::evaluate(fixture).expect("RR-0738: Extended: Journal index compaction wire planner v23");
    let second = relayring::capabilities::rr_0738_journal_index_compaction_extended::evaluate(fixture).expect("RR-0738: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0738: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0738: scanner should emit domain hints");
}
