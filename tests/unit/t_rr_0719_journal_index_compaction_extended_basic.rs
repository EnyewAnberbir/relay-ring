//! Integration test for `RR-0719` (basic).
//! Extended: Journal index compaction optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0719_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd6, 0xd8];
    let first = relayring::capabilities::rr_0719_journal_index_compaction_extended::evaluate(fixture).expect("RR-0719: Extended: Journal index compaction optimize registry v4");
    let second = relayring::capabilities::rr_0719_journal_index_compaction_extended::evaluate(fixture).expect("RR-0719: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0719: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0719: window consumes the whole buffer");
}
