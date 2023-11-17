//! Integration test for `RR-0743` (basic).
//! Extended: Journal index compaction refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0743_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xee, 0xf0];
    let first = relayring::capabilities::rr_0743_journal_index_compaction_extended::evaluate(fixture).expect("RR-0743: Extended: Journal index compaction refactor mutator v28");
    let second = relayring::capabilities::rr_0743_journal_index_compaction_extended::evaluate(fixture).expect("RR-0743: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0743: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0743: stats visits every byte");
}
