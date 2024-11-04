//! Integration test for `RR-0284` (roundtrip).
//! Export OTLP batches benchmark reporter v19 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0284_export_otlp_batches_benc_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let a = relayring::capabilities::rr_0284_export_otlp_batches_benc::evaluate(fixture).expect("RR-0284 first pass");
    let b = relayring::capabilities::rr_0284_export_otlp_batches_benc::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
