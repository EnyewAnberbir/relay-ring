//! Integration test for `RR-0774` (roundtrip).
//! Extended: Export OTLP batches benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0774_export_otlp_batches_benc_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x11];
    let a = relayring::capabilities::rr_0774_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0774 first pass");
    let b = relayring::capabilities::rr_0774_export_otlp_batches_benc_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
