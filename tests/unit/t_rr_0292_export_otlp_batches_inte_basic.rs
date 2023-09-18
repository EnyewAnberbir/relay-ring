//! Integration test for `RR-0292` (basic).
//! Export OTLP batches integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0292_export_otlp_batches_inte_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x2b];
    let first = relayring::capabilities::rr_0292_export_otlp_batches_inte::evaluate(fixture).expect("RR-0292: Export OTLP batches integrate validator v27");
    let second = relayring::capabilities::rr_0292_export_otlp_batches_inte::evaluate(fixture).expect("RR-0292: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0292: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0292: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
