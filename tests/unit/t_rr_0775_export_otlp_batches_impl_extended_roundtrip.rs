//! Integration test for `RR-0775` (roundtrip).
//! Extended: Export OTLP batches implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0775_export_otlp_batches_impl_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12];
    let a = relayring::capabilities::rr_0775_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0775 first pass");
    let b = relayring::capabilities::rr_0775_export_otlp_batches_impl_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
