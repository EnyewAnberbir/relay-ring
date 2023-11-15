//! Integration test for `RR-0717` (basic).
//! Extended: Journal index compaction harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0717_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd4, 0xd6];
    let first = relayring::capabilities::rr_0717_journal_index_compaction_extended::evaluate(fixture).expect("RR-0717: Extended: Journal index compaction harden index v2");
    let second = relayring::capabilities::rr_0717_journal_index_compaction_extended::evaluate(fixture).expect("RR-0717: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0717: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0717: window consumes the whole buffer");
}
