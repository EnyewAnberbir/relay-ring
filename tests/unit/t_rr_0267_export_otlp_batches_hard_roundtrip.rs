//! Integration test for `RR-0267` (roundtrip).
//! Export OTLP batches harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0267_export_otlp_batches_hard_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12];
    let a = relayring::capabilities::rr_0267_export_otlp_batches_hard::evaluate(fixture).expect("RR-0267 first pass");
    let b = relayring::capabilities::rr_0267_export_otlp_batches_hard::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
