//! Integration test for `RR-0749` (basic).
//! Extended: Journal index compaction optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0749_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf4, 0xf6];
    let first = relayring::capabilities::rr_0749_journal_index_compaction_extended::evaluate(fixture).expect("RR-0749: Extended: Journal index compaction optimize registry v34");
    let second = relayring::capabilities::rr_0749_journal_index_compaction_extended::evaluate(fixture).expect("RR-0749: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0749: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0749: window consumes the whole buffer");
}
