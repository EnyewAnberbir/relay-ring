//! Integration test for `RR-0218` (basic).
//! Journal index compaction wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0218_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdd, 0xdf];
    let first = relayring::capabilities::rr_0218_journal_index_compaction::evaluate(fixture).expect("RR-0218: Journal index compaction wire planner v3");
    let second = relayring::capabilities::rr_0218_journal_index_compaction::evaluate(fixture).expect("RR-0218: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0218: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0218: window consumes the whole buffer");
}
