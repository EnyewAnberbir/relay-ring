//! Integration test for `RR-0276` (roundtrip).
//! Export OTLP batches extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0276_export_otlp_batches_exte_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x19, 0x1b];
    let a = relayring::capabilities::rr_0276_export_otlp_batches_exte::evaluate(fixture).expect("RR-0276 first pass");
    let b = relayring::capabilities::rr_0276_export_otlp_batches_exte::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
