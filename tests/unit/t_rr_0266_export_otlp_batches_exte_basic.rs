//! Integration test for `RR-0266` (basic).
//! Export OTLP batches extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0266_export_otlp_batches_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x11];
    let first = relayring::capabilities::rr_0266_export_otlp_batches_exte::evaluate(fixture).expect("RR-0266: Export OTLP batches extend codec v1");
    let second = relayring::capabilities::rr_0266_export_otlp_batches_exte::evaluate(fixture).expect("RR-0266: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0266: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0266: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
