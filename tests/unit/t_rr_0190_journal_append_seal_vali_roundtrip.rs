//! Integration test for `RR-0190` (roundtrip).
//! Journal append seal validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0190_journal_append_seal_vali_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc1, 0xc3];
    let a = relayring::capabilities::rr_0190_journal_append_seal_vali::evaluate(fixture).expect("RR-0190 first pass");
    let b = relayring::capabilities::rr_0190_journal_append_seal_vali::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
