//! Integration test for `RR-0791` (empty).
//! Extended: Export OTLP batches export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0791_export_otlp_batches_expo_extended_empty() {
    assert!(relayring::capabilities::rr_0791_export_otlp_batches_expo_extended::evaluate(&[]).is_err(), "RR-0791: empty input must fail for Extended: Export OTLP batches export adapter v26");
}
