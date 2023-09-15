//! Integration test for `RR-0283` (basic).
//! Export OTLP batches refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0283_export_otlp_batches_refa_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let first = relayring::capabilities::rr_0283_export_otlp_batches_refa::evaluate(fixture).expect("RR-0283: Export OTLP batches refactor mutator v18");
    let second = relayring::capabilities::rr_0283_export_otlp_batches_refa::evaluate(fixture).expect("RR-0283: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0283: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0283: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
