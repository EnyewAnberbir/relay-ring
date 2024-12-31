//! Integration test for `RR-0710` (roundtrip).
//! Extended: Journal append seal validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0710_journal_append_seal_vali_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xcd, 0xcf];
    let a = relayring::capabilities::rr_0710_journal_append_seal_vali_extended::evaluate(fixture).expect("RR-0710 first pass");
    let b = relayring::capabilities::rr_0710_journal_append_seal_vali_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
