//! Integration test for `RR-0785` (basic).
//! Extended: Export OTLP batches implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0785_export_otlp_batches_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1a, 0x1c];
    let first = relayring::capabilities::rr_0785_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0785: Extended: Export OTLP batches implement pipeline v20");
    let second = relayring::capabilities::rr_0785_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0785: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0785: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0785: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
