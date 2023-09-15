//! Integration test for `RR-0282` (basic).
//! Export OTLP batches integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0282_export_otlp_batches_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let first = relayring::capabilities::rr_0282_export_otlp_batches_inte::evaluate(fixture).expect("RR-0282: Export OTLP batches integrate validator v17");
    let second = relayring::capabilities::rr_0282_export_otlp_batches_inte::evaluate(fixture).expect("RR-0282: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0282: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0282: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
