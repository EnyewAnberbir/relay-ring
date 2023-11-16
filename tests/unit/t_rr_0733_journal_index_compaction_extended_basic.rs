//! Integration test for `RR-0733` (basic).
//! Extended: Journal index compaction refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0733_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe4, 0xe6];
    let first = relayring::capabilities::rr_0733_journal_index_compaction_extended::evaluate(fixture).expect("RR-0733: Extended: Journal index compaction refactor mutator v18");
    let second = relayring::capabilities::rr_0733_journal_index_compaction_extended::evaluate(fixture).expect("RR-0733: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0733: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0733: scanner should emit domain hints");
}
