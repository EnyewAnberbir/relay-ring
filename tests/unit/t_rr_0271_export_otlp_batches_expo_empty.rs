//! Integration test for `RR-0271` (empty).
//! Export OTLP batches export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0271_export_otlp_batches_expo_empty() {
    assert!(relayring::capabilities::rr_0271_export_otlp_batches_expo::evaluate(&[]).is_err(), "RR-0271: empty input must fail for Export OTLP batches export adapter v6");
}
