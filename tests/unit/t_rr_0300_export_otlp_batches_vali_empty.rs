//! Integration test for `RR-0300` (empty).
//! Export OTLP batches validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0300_export_otlp_batches_vali_empty() {
    assert!(relayring::capabilities::rr_0300_export_otlp_batches_vali::evaluate(&[]).is_err(), "RR-0300: empty input must fail for Export OTLP batches validate resolver v35");
}
