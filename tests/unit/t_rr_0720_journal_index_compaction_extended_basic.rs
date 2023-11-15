//! Integration test for `RR-0720` (basic).
//! Extended: Journal index compaction validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0720_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd7, 0xd9];
    let first = relayring::capabilities::rr_0720_journal_index_compaction_extended::evaluate(fixture).expect("RR-0720: Extended: Journal index compaction validate resolver v5");
    let second = relayring::capabilities::rr_0720_journal_index_compaction_extended::evaluate(fixture).expect("RR-0720: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0720: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0720: window consumes the whole buffer");
}
