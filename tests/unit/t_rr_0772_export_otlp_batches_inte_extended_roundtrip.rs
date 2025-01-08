//! Integration test for `RR-0772` (roundtrip).
//! Extended: Export OTLP batches integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0772_export_otlp_batches_inte_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0d, 0x0f];
    let a = relayring::capabilities::rr_0772_export_otlp_batches_inte_extended::evaluate(fixture).expect("RR-0772 first pass");
    let b = relayring::capabilities::rr_0772_export_otlp_batches_inte_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
