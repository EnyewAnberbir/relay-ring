//! Integration test for `RR-0216` (basic).
//! Journal index compaction extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0216_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdb, 0xdd];
    let first = relayring::capabilities::rr_0216_journal_index_compaction::evaluate(fixture).expect("RR-0216: Journal index compaction extend codec v1");
    let second = relayring::capabilities::rr_0216_journal_index_compaction::evaluate(fixture).expect("RR-0216: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0216: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0216: window consumes the whole buffer");
}
