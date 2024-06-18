//! Integration test for `RR-0289` (empty).
//! Export OTLP batches optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0289_export_otlp_batches_opti_empty() {
    assert!(relayring::capabilities::rr_0289_export_otlp_batches_opti::evaluate(&[]).is_err(), "RR-0289: empty input must fail for Export OTLP batches optimize registry v24");
}
