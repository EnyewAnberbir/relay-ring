//! Integration test for `RR-0277` (empty).
//! Export OTLP batches harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0277_export_otlp_batches_hard_empty() {
    assert!(relayring::capabilities::rr_0277_export_otlp_batches_hard::evaluate(&[]).is_err(), "RR-0277: empty input must fail for Export OTLP batches harden index v12");
}
