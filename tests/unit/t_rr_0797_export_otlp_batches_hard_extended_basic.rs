//! Integration test for `RR-0797` (basic).
//! Extended: Export OTLP batches harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0797_export_otlp_batches_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x26, 0x28];
    let first = relayring::capabilities::rr_0797_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0797: Extended: Export OTLP batches harden index v32");
    let second = relayring::capabilities::rr_0797_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0797: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0797: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0797: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
