//! Integration test for `RR-0280` (basic).
//! Export OTLP batches validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0280_export_otlp_batches_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1d, 0x1f];
    let first = relayring::capabilities::rr_0280_export_otlp_batches_vali::evaluate(fixture).expect("RR-0280: Export OTLP batches validate resolver v15");
    let second = relayring::capabilities::rr_0280_export_otlp_batches_vali::evaluate(fixture).expect("RR-0280: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0280: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0280: stats visits every byte");
}
