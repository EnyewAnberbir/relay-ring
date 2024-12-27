//! Integration test for `RR-0685` (roundtrip).
//! Extended: Journal append seal implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0685_journal_append_seal_impl_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xb4, 0xb6];
    let a = relayring::capabilities::rr_0685_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0685 first pass");
    let b = relayring::capabilities::rr_0685_journal_append_seal_impl_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
