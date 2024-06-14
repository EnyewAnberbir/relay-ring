//! Integration test for `RR-0276` (empty).
//! Export OTLP batches extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0276_export_otlp_batches_exte_empty() {
    assert!(relayring::capabilities::rr_0276_export_otlp_batches_exte::evaluate(&[]).is_err(), "RR-0276: empty input must fail for Export OTLP batches extend codec v11");
}
