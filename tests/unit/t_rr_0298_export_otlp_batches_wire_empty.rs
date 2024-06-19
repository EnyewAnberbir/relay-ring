//! Integration test for `RR-0298` (empty).
//! Export OTLP batches wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0298_export_otlp_batches_wire_empty() {
    assert!(relayring::capabilities::rr_0298_export_otlp_batches_wire::evaluate(&[]).is_err(), "RR-0298: empty input must fail for Export OTLP batches wire planner v33");
}
