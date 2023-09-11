//! Integration test for `RR-0239` (basic).
//! Journal index compaction optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0239_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf2, 0xf4];
    let first = relayring::capabilities::rr_0239_journal_index_compaction::evaluate(fixture).expect("RR-0239: Journal index compaction optimize registry v24");
    let second = relayring::capabilities::rr_0239_journal_index_compaction::evaluate(fixture).expect("RR-0239: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0239: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0239: window consumes the whole buffer");
}
