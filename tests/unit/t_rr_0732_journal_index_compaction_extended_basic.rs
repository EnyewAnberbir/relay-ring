//! Integration test for `RR-0732` (basic).
//! Extended: Journal index compaction integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0732_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe3, 0xe5];
    let first = relayring::capabilities::rr_0732_journal_index_compaction_extended::evaluate(fixture).expect("RR-0732: Extended: Journal index compaction integrate validator v17");
    let second = relayring::capabilities::rr_0732_journal_index_compaction_extended::evaluate(fixture).expect("RR-0732: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0732: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0732: stats visits every byte");
}
