//! Integration test for `RR-0214` (roundtrip).
//! Journal append seal benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0214_journal_append_seal_benc_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd9, 0xdb];
    let a = relayring::capabilities::rr_0214_journal_append_seal_benc::evaluate(fixture).expect("RR-0214 first pass");
    let b = relayring::capabilities::rr_0214_journal_append_seal_benc::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
