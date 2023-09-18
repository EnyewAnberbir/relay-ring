//! Integration test for `RR-0298` (basic).
//! Export OTLP batches wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0298_export_otlp_batches_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x31];
    let first = relayring::capabilities::rr_0298_export_otlp_batches_wire::evaluate(fixture).expect("RR-0298: Export OTLP batches wire planner v33");
    let second = relayring::capabilities::rr_0298_export_otlp_batches_wire::evaluate(fixture).expect("RR-0298: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0298: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0298: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
