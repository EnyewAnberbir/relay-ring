//! Integration test for `RR-0283` (roundtrip).
//! Export OTLP batches refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0283_export_otlp_batches_refa_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let a = relayring::capabilities::rr_0283_export_otlp_batches_refa::evaluate(fixture).expect("RR-0283 first pass");
    let b = relayring::capabilities::rr_0283_export_otlp_batches_refa::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
