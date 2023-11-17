//! Integration test for `RR-0746` (basic).
//! Extended: Journal index compaction extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0746_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf1, 0xf3];
    let first = relayring::capabilities::rr_0746_journal_index_compaction_extended::evaluate(fixture).expect("RR-0746: Extended: Journal index compaction extend codec v31");
    let second = relayring::capabilities::rr_0746_journal_index_compaction_extended::evaluate(fixture).expect("RR-0746: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0746: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0746: scanner should emit domain hints");
}
