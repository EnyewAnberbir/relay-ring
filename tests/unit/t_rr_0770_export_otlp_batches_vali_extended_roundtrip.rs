//! Integration test for `RR-0770` (roundtrip).
//! Extended: Export OTLP batches validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0770_export_otlp_batches_vali_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x0d];
    let a = relayring::capabilities::rr_0770_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0770 first pass");
    let b = relayring::capabilities::rr_0770_export_otlp_batches_vali_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
