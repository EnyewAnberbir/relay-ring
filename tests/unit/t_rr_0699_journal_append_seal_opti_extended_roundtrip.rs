//! Integration test for `RR-0699` (roundtrip).
//! Extended: Journal append seal optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0699_journal_append_seal_opti_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc2, 0xc4];
    let a = relayring::capabilities::rr_0699_journal_append_seal_opti_extended::evaluate(fixture).expect("RR-0699 first pass");
    let b = relayring::capabilities::rr_0699_journal_append_seal_opti_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
