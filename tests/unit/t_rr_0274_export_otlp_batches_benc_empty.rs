//! Integration test for `RR-0274` (empty).
//! Export OTLP batches benchmark reporter v9 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0274_export_otlp_batches_benc_empty() {
    assert!(relayring::capabilities::rr_0274_export_otlp_batches_benc::evaluate(&[]).is_err(), "RR-0274: empty input must fail for Export OTLP batches benchmark reporter v9");
}
