//! Integration test for `RR-0222` (basic).
//! Journal index compaction integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0222_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe1, 0xe3];
    let first = relayring::capabilities::rr_0222_journal_index_compaction::evaluate(fixture).expect("RR-0222: Journal index compaction integrate validator v7");
    let second = relayring::capabilities::rr_0222_journal_index_compaction::evaluate(fixture).expect("RR-0222: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0222: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0222: window consumes the whole buffer");
}
