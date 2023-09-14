//! Integration test for `RR-0274` (basic).
//! Export OTLP batches benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0274_export_otlp_batches_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x19];
    let first = relayring::capabilities::rr_0274_export_otlp_batches_benc::evaluate(fixture).expect("RR-0274: Export OTLP batches benchmark reporter v9");
    let second = relayring::capabilities::rr_0274_export_otlp_batches_benc::evaluate(fixture).expect("RR-0274: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0274: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0274: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
