//! Integration test for `RR-0714` (roundtrip).
//! Extended: Journal append seal benchmark reporter v39 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0714_journal_append_seal_benc_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd1, 0xd3];
    let a = relayring::capabilities::rr_0714_journal_append_seal_benc_extended::evaluate(fixture).expect("RR-0714 first pass");
    let b = relayring::capabilities::rr_0714_journal_append_seal_benc_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
