//! Integration test for `RR-0798` (empty).
//! Extended: Export OTLP batches wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0798_export_otlp_batches_wire_extended_empty() {
    assert!(relayring::capabilities::rr_0798_export_otlp_batches_wire_extended::evaluate(&[]).is_err(), "RR-0798: empty input must fail for Extended: Export OTLP batches wire planner v33");
}
