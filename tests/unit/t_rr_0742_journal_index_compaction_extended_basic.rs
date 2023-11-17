//! Integration test for `RR-0742` (basic).
//! Extended: Journal index compaction integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0742_journal_index_compaction_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xed, 0xef];
    let first = relayring::capabilities::rr_0742_journal_index_compaction_extended::evaluate(fixture).expect("RR-0742: Extended: Journal index compaction integrate validator v27");
    let second = relayring::capabilities::rr_0742_journal_index_compaction_extended::evaluate(fixture).expect("RR-0742: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0742: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0742: stats visits every byte");
}
