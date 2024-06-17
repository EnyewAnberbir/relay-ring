//! Integration test for `RR-0281` (empty).
//! Export OTLP batches export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0281_export_otlp_batches_expo_empty() {
    assert!(relayring::capabilities::rr_0281_export_otlp_batches_expo::evaluate(&[]).is_err(), "RR-0281: empty input must fail for Export OTLP batches export adapter v16");
}
