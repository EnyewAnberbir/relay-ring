//! Integration test for `RR-0695` (roundtrip).
//! Extended: Journal append seal implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0695_journal_append_seal_impl_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbe, 0xc0];
    let a = relayring::capabilities::rr_0695_journal_append_seal_impl_extended::evaluate(fixture).expect("RR-0695 first pass");
    let b = relayring::capabilities::rr_0695_journal_append_seal_impl_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
