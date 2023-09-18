//! Integration test for `RR-0290` (basic).
//! Export OTLP batches validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0290_export_otlp_batches_vali_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let first = relayring::capabilities::rr_0290_export_otlp_batches_vali::evaluate(fixture).expect("RR-0290: Export OTLP batches validate resolver v25");
    let second = relayring::capabilities::rr_0290_export_otlp_batches_vali::evaluate(fixture).expect("RR-0290: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0290: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0290: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
