//! Integration test for `RR-0277` (basic).
//! Export OTLP batches harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0277_export_otlp_batches_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let first = relayring::capabilities::rr_0277_export_otlp_batches_hard::evaluate(fixture).expect("RR-0277: Export OTLP batches harden index v12");
    let second = relayring::capabilities::rr_0277_export_otlp_batches_hard::evaluate(fixture).expect("RR-0277: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0277: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0277: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
