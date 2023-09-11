//! Integration test for `RR-0240` (basic).
//! Journal index compaction validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0240_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf3, 0xf5];
    let first = relayring::capabilities::rr_0240_journal_index_compaction::evaluate(fixture).expect("RR-0240: Journal index compaction validate resolver v25");
    let second = relayring::capabilities::rr_0240_journal_index_compaction::evaluate(fixture).expect("RR-0240: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0240: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0240: window consumes the whole buffer");
}
