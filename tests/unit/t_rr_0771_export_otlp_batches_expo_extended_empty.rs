//! Integration test for `RR-0771` (empty).
//! Extended: Export OTLP batches export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0771_export_otlp_batches_expo_extended_empty() {
    assert!(relayring::capabilities::rr_0771_export_otlp_batches_expo_extended::evaluate(&[]).is_err(), "RR-0771: empty input must fail for Extended: Export OTLP batches export adapter v6");
}
