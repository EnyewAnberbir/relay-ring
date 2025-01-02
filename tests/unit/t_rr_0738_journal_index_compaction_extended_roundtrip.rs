//! Integration test for `RR-0738` (roundtrip).
//! Extended: Journal index compaction wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0738_journal_index_compaction_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe9, 0xeb];
    let a = relayring::capabilities::rr_0738_journal_index_compaction_extended::evaluate(fixture).expect("RR-0738 first pass");
    let b = relayring::capabilities::rr_0738_journal_index_compaction_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
