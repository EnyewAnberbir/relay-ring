//! Integration test for `RR-0197` (roundtrip).
//! Journal append seal harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0197_journal_append_seal_hard_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc8, 0xca];
    let a = relayring::capabilities::rr_0197_journal_append_seal_hard::evaluate(fixture).expect("RR-0197 first pass");
    let b = relayring::capabilities::rr_0197_journal_append_seal_hard::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
