//! Integration test for `RR-0290` (empty).
//! Export OTLP batches validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0290_export_otlp_batches_vali_empty() {
    assert!(relayring::capabilities::rr_0290_export_otlp_batches_vali::evaluate(&[]).is_err(), "RR-0290: empty input must fail for Export OTLP batches validate resolver v25");
}
