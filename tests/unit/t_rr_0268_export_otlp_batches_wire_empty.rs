//! Integration test for `RR-0268` (empty).
//! Export OTLP batches wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0268_export_otlp_batches_wire_empty() {
    assert!(relayring::capabilities::rr_0268_export_otlp_batches_wire::evaluate(&[]).is_err(), "RR-0268: empty input must fail for Export OTLP batches wire planner v3");
}
