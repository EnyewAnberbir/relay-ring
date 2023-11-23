//! Integration test for `RR-0789` (basic).
//! Extended: Export OTLP batches optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0789_export_otlp_batches_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let first = relayring::capabilities::rr_0789_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0789: Extended: Export OTLP batches optimize registry v24");
    let second = relayring::capabilities::rr_0789_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0789: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0789: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0789: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
