//! Integration test for `RR-0278` (basic).
//! Export OTLP batches wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0278_export_otlp_batches_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1b, 0x1d];
    let first = relayring::capabilities::rr_0278_export_otlp_batches_wire::evaluate(fixture).expect("RR-0278: Export OTLP batches wire planner v13");
    let second = relayring::capabilities::rr_0278_export_otlp_batches_wire::evaluate(fixture).expect("RR-0278: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0278: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0278: scanner should emit domain hints");
}
