//! Integration test for `RR-0733` (roundtrip).
//! Extended: Journal index compaction refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0733_journal_index_compaction_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe4, 0xe6];
    let a = relayring::capabilities::rr_0733_journal_index_compaction_extended::evaluate(fixture).expect("RR-0733 first pass");
    let b = relayring::capabilities::rr_0733_journal_index_compaction_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
