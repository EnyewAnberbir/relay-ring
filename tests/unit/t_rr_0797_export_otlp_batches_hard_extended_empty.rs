//! Integration test for `RR-0797` (empty).
//! Extended: Export OTLP batches harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0797_export_otlp_batches_hard_extended_empty() {
    assert!(relayring::capabilities::rr_0797_export_otlp_batches_hard_extended::evaluate(&[]).is_err(), "RR-0797: empty input must fail for Extended: Export OTLP batches harden index v32");
}
