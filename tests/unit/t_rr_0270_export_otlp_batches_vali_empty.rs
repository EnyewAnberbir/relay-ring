//! Integration test for `RR-0270` (empty).
//! Export OTLP batches validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0270_export_otlp_batches_vali_empty() {
    assert!(relayring::capabilities::rr_0270_export_otlp_batches_vali::evaluate(&[]).is_err(), "RR-0270: empty input must fail for Export OTLP batches validate resolver v5");
}
