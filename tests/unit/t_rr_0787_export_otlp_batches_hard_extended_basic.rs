//! Integration test for `RR-0787` (basic).
//! Extended: Export OTLP batches harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0787_export_otlp_batches_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x1e];
    let first = relayring::capabilities::rr_0787_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0787: Extended: Export OTLP batches harden index v22");
    let second = relayring::capabilities::rr_0787_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0787: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0787: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0787: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
