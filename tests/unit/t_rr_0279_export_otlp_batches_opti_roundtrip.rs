//! Integration test for `RR-0279` (roundtrip).
//! Export OTLP batches optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0279_export_otlp_batches_opti_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let a = relayring::capabilities::rr_0279_export_otlp_batches_opti::evaluate(fixture).expect("RR-0279 first pass");
    let b = relayring::capabilities::rr_0279_export_otlp_batches_opti::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
