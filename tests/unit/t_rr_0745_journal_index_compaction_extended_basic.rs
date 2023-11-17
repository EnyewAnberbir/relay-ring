//! Integration test for `RR-0745` (basic).
//! Extended: Journal index compaction implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0745_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0xf2];
    let first = relayring::capabilities::rr_0745_journal_index_compaction_extended::evaluate(fixture).expect("RR-0745: Extended: Journal index compaction implement pipeline v30");
    let second = relayring::capabilities::rr_0745_journal_index_compaction_extended::evaluate(fixture).expect("RR-0745: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0745: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0745: stats visits every byte");
}
