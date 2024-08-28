//! Integration test for `RR-0787` (empty).
//! Extended: Export OTLP batches harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0787_export_otlp_batches_hard_extended_empty() {
    assert!(relayring::capabilities::rr_0787_export_otlp_batches_hard_extended::evaluate(&[]).is_err(), "RR-0787: empty input must fail for Extended: Export OTLP batches harden index v22");
}
