//! Integration test for `RR-0778` (empty).
//! Extended: Export OTLP batches wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0778_export_otlp_batches_wire_extended_empty() {
    assert!(relayring::capabilities::rr_0778_export_otlp_batches_wire_extended::evaluate(&[]).is_err(), "RR-0778: empty input must fail for Extended: Export OTLP batches wire planner v13");
}
