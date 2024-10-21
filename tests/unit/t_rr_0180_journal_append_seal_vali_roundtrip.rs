//! Integration test for `RR-0180` (roundtrip).
//! Journal append seal validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0180_journal_append_seal_vali_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb7, 0xb9];
    let a = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(fixture).expect("RR-0180 first pass");
    let b = relayring::capabilities::rr_0180_journal_append_seal_vali::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
