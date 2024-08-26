//! Integration test for `RR-0767` (empty).
//! Extended: Export OTLP batches harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0767_export_otlp_batches_hard_extended_empty() {
    assert!(relayring::capabilities::rr_0767_export_otlp_batches_hard_extended::evaluate(&[]).is_err(), "RR-0767: empty input must fail for Extended: Export OTLP batches harden index v2");
}
