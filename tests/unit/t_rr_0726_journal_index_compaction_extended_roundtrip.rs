//! Integration test for `RR-0726` (roundtrip).
//! Extended: Journal index compaction extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0726_journal_index_compaction_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdd, 0xdf];
    let a = relayring::capabilities::rr_0726_journal_index_compaction_extended::evaluate(fixture).expect("RR-0726 first pass");
    let b = relayring::capabilities::rr_0726_journal_index_compaction_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
