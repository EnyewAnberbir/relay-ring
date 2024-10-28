//! Integration test for `RR-0243` (roundtrip).
//! Journal index compaction refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0243_journal_index_compaction_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf6, 0xf8];
    let a = relayring::capabilities::rr_0243_journal_index_compaction::evaluate(fixture).expect("RR-0243 first pass");
    let b = relayring::capabilities::rr_0243_journal_index_compaction::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
