//! Integration test for `RR-0748` (basic).
//! Extended: Journal index compaction wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0748_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf3, 0xf5];
    let first = relayring::capabilities::rr_0748_journal_index_compaction_extended::evaluate(fixture).expect("RR-0748: Extended: Journal index compaction wire planner v33");
    let second = relayring::capabilities::rr_0748_journal_index_compaction_extended::evaluate(fixture).expect("RR-0748: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0748: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0748: scanner should emit domain hints");
}
