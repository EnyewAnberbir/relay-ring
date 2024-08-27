//! Integration test for `RR-0781` (empty).
//! Extended: Export OTLP batches export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0781_export_otlp_batches_expo_extended_empty() {
    assert!(relayring::capabilities::rr_0781_export_otlp_batches_expo_extended::evaluate(&[]).is_err(), "RR-0781: empty input must fail for Extended: Export OTLP batches export adapter v16");
}
