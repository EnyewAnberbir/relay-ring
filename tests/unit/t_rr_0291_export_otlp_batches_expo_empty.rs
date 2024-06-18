//! Integration test for `RR-0291` (empty).
//! Export OTLP batches export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0291_export_otlp_batches_expo_empty() {
    assert!(relayring::capabilities::rr_0291_export_otlp_batches_expo::evaluate(&[]).is_err(), "RR-0291: empty input must fail for Export OTLP batches export adapter v26");
}
