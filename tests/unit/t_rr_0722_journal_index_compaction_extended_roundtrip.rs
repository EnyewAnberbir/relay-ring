//! Integration test for `RR-0722` (roundtrip).
//! Extended: Journal index compaction integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0722_journal_index_compaction_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd9, 0xdb];
    let a = relayring::capabilities::rr_0722_journal_index_compaction_extended::evaluate(fixture).expect("RR-0722 first pass");
    let b = relayring::capabilities::rr_0722_journal_index_compaction_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
