//! Integration test for `RR-0213` (roundtrip).
//! Journal append seal refactor mutator v38 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0213_journal_append_seal_refa_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd8, 0xda];
    let a = relayring::capabilities::rr_0213_journal_append_seal_refa::evaluate(fixture).expect("RR-0213 first pass");
    let b = relayring::capabilities::rr_0213_journal_append_seal_refa::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
