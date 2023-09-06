//! Integration test for `RR-0217` (basic).
//! Journal index compaction harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0217_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdc, 0xde];
    let first = relayring::capabilities::rr_0217_journal_index_compaction::evaluate(fixture).expect("RR-0217: Journal index compaction harden index v2");
    let second = relayring::capabilities::rr_0217_journal_index_compaction::evaluate(fixture).expect("RR-0217: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0217: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0217: window consumes the whole buffer");
}
