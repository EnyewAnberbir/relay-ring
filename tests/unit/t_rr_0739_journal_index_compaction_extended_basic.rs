//! Integration test for `RR-0739` (basic).
//! Extended: Journal index compaction optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0739_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xea, 0xec];
    let first = relayring::capabilities::rr_0739_journal_index_compaction_extended::evaluate(fixture).expect("RR-0739: Extended: Journal index compaction optimize registry v24");
    let second = relayring::capabilities::rr_0739_journal_index_compaction_extended::evaluate(fixture).expect("RR-0739: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0739: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0739: stats visits every byte");
}
