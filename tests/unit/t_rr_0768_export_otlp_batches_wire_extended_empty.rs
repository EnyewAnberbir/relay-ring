//! Integration test for `RR-0768` (empty).
//! Extended: Export OTLP batches wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0768_export_otlp_batches_wire_extended_empty() {
    assert!(relayring::capabilities::rr_0768_export_otlp_batches_wire_extended::evaluate(&[]).is_err(), "RR-0768: empty input must fail for Extended: Export OTLP batches wire planner v3");
}
