//! Integration test for `RR-0287` (empty).
//! Export OTLP batches harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0287_export_otlp_batches_hard_empty() {
    assert!(relayring::capabilities::rr_0287_export_otlp_batches_hard::evaluate(&[]).is_err(), "RR-0287: empty input must fail for Export OTLP batches harden index v22");
}
