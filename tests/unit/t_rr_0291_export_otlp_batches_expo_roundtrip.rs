//! Integration test for `RR-0291` (roundtrip).
//! Export OTLP batches export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0291_export_otlp_batches_expo_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x2a];
    let a = relayring::capabilities::rr_0291_export_otlp_batches_expo::evaluate(fixture).expect("RR-0291 first pass");
    let b = relayring::capabilities::rr_0291_export_otlp_batches_expo::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
