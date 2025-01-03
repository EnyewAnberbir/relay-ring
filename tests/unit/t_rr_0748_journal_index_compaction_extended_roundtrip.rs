//! Integration test for `RR-0748` (roundtrip).
//! Extended: Journal index compaction wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0748_journal_index_compaction_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf3, 0xf5];
    let a = relayring::capabilities::rr_0748_journal_index_compaction_extended::evaluate(fixture).expect("RR-0748 first pass");
    let b = relayring::capabilities::rr_0748_journal_index_compaction_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
