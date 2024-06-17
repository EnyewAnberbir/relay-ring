//! Integration test for `RR-0280` (empty).
//! Export OTLP batches validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0280_export_otlp_batches_vali_empty() {
    assert!(relayring::capabilities::rr_0280_export_otlp_batches_vali::evaluate(&[]).is_err(), "RR-0280: empty input must fail for Export OTLP batches validate resolver v15");
}
