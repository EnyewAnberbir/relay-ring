//! Integration test for `RR-0250` (roundtrip).
//! Journal index compaction validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0250_journal_index_compaction_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfd, 0x01];
    let a = relayring::capabilities::rr_0250_journal_index_compaction::evaluate(fixture).expect("RR-0250 first pass");
    let b = relayring::capabilities::rr_0250_journal_index_compaction::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
