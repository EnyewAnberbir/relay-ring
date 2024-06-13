//! Integration test for `RR-0267` (empty).
//! Export OTLP batches harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0267_export_otlp_batches_hard_empty() {
    assert!(relayring::capabilities::rr_0267_export_otlp_batches_hard::evaluate(&[]).is_err(), "RR-0267: empty input must fail for Export OTLP batches harden index v2");
}
