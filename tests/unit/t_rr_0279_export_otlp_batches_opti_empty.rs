//! Integration test for `RR-0279` (empty).
//! Export OTLP batches optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0279_export_otlp_batches_opti_empty() {
    assert!(relayring::capabilities::rr_0279_export_otlp_batches_opti::evaluate(&[]).is_err(), "RR-0279: empty input must fail for Export OTLP batches optimize registry v14");
}
