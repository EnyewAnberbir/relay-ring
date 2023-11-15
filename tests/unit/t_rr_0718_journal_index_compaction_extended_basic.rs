//! Integration test for `RR-0718` (basic).
//! Extended: Journal index compaction wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0718_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd5, 0xd7];
    let first = relayring::capabilities::rr_0718_journal_index_compaction_extended::evaluate(fixture).expect("RR-0718: Extended: Journal index compaction wire planner v3");
    let second = relayring::capabilities::rr_0718_journal_index_compaction_extended::evaluate(fixture).expect("RR-0718: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0718: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0718: stats visits every byte");
}
