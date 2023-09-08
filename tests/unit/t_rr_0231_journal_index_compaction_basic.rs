//! Integration test for `RR-0231` (basic).
//! Journal index compaction export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0231_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xea, 0xec];
    let first = relayring::capabilities::rr_0231_journal_index_compaction::evaluate(fixture).expect("RR-0231: Journal index compaction export adapter v16");
    let second = relayring::capabilities::rr_0231_journal_index_compaction::evaluate(fixture).expect("RR-0231: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0231: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0231: window consumes the whole buffer");
}
