//! Integration test for `RR-0729` (basic).
//! Extended: Journal index compaction optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0729_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0xe2];
    let first = relayring::capabilities::rr_0729_journal_index_compaction_extended::evaluate(fixture).expect("RR-0729: Extended: Journal index compaction optimize registry v14");
    let second = relayring::capabilities::rr_0729_journal_index_compaction_extended::evaluate(fixture).expect("RR-0729: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0729: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0729: window consumes the whole buffer");
}
