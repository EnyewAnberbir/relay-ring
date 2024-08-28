//! Integration test for `RR-0788` (empty).
//! Extended: Export OTLP batches wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0788_export_otlp_batches_wire_extended_empty() {
    assert!(relayring::capabilities::rr_0788_export_otlp_batches_wire_extended::evaluate(&[]).is_err(), "RR-0788: empty input must fail for Extended: Export OTLP batches wire planner v23");
}
