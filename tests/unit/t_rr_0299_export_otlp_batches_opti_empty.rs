//! Integration test for `RR-0299` (empty).
//! Export OTLP batches optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0299_export_otlp_batches_opti_empty() {
    assert!(relayring::capabilities::rr_0299_export_otlp_batches_opti::evaluate(&[]).is_err(), "RR-0299: empty input must fail for Export OTLP batches optimize registry v34");
}
