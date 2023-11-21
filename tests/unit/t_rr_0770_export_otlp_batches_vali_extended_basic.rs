//! Integration test for `RR-0770` (basic).
//! Extended: Export OTLP batches validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0770_export_otlp_batches_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0b, 0x0d];
    let first = relayring::capabilities::rr_0770_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0770: Extended: Export OTLP batches validate resolver v5");
    let second = relayring::capabilities::rr_0770_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0770: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0770: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0770: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
