//! Integration test for `RR-0723` (basic).
//! Extended: Journal index compaction refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0723_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xda, 0xdc];
    let first = relayring::capabilities::rr_0723_journal_index_compaction_extended::evaluate(fixture).expect("RR-0723: Extended: Journal index compaction refactor mutator v8");
    let second = relayring::capabilities::rr_0723_journal_index_compaction_extended::evaluate(fixture).expect("RR-0723: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0723: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0723: stats visits every byte");
}
