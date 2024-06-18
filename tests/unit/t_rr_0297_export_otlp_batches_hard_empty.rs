//! Integration test for `RR-0297` (empty).
//! Export OTLP batches harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0297_export_otlp_batches_hard_empty() {
    assert!(relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(&[]).is_err(), "RR-0297: empty input must fail for Export OTLP batches harden index v32");
}
