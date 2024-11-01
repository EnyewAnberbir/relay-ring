//! Integration test for `RR-0275` (roundtrip).
//! Export OTLP batches implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0275_export_otlp_batches_impl_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let a = relayring::capabilities::rr_0275_export_otlp_batches_impl::evaluate(fixture).expect("RR-0275 first pass");
    let b = relayring::capabilities::rr_0275_export_otlp_batches_impl::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
