//! Integration test for `RR-0721` (basic).
//! Extended: Journal index compaction export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0721_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let first = relayring::capabilities::rr_0721_journal_index_compaction_extended::evaluate(fixture).expect("RR-0721: Extended: Journal index compaction export adapter v6");
    let second = relayring::capabilities::rr_0721_journal_index_compaction_extended::evaluate(fixture).expect("RR-0721: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0721: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0721: window consumes the whole buffer");
}
