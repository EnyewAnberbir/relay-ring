//! Integration test for `RR-0286` (basic).
//! Export OTLP batches extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0286_export_otlp_batches_exte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let first = relayring::capabilities::rr_0286_export_otlp_batches_exte::evaluate(fixture).expect("RR-0286: Export OTLP batches extend codec v21");
    let second = relayring::capabilities::rr_0286_export_otlp_batches_exte::evaluate(fixture).expect("RR-0286: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0286: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0286: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
