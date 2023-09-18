//! Integration test for `RR-0294` (basic).
//! Export OTLP batches benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0294_export_otlp_batches_benc_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x2d];
    let first = relayring::capabilities::rr_0294_export_otlp_batches_benc::evaluate(fixture).expect("RR-0294: Export OTLP batches benchmark reporter v29");
    let second = relayring::capabilities::rr_0294_export_otlp_batches_benc::evaluate(fixture).expect("RR-0294: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0294: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0294: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
