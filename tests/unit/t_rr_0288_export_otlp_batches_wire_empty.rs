//! Integration test for `RR-0288` (empty).
//! Export OTLP batches wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0288_export_otlp_batches_wire_empty() {
    assert!(relayring::capabilities::rr_0288_export_otlp_batches_wire::evaluate(&[]).is_err(), "RR-0288: empty input must fail for Export OTLP batches wire planner v23");
}
