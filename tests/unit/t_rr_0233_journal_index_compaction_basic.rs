//! Integration test for `RR-0233` (basic).
//! Journal index compaction refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0233_journal_index_compaction_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xec, 0xee];
    let first = relayring::capabilities::rr_0233_journal_index_compaction::evaluate(fixture).expect("RR-0233: Journal index compaction refactor mutator v18");
    let second = relayring::capabilities::rr_0233_journal_index_compaction::evaluate(fixture).expect("RR-0233: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0233: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0233: window consumes the whole buffer");
}
