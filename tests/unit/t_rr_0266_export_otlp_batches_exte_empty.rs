//! Integration test for `RR-0266` (empty).
//! Export OTLP batches extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0266_export_otlp_batches_exte_empty() {
    assert!(relayring::capabilities::rr_0266_export_otlp_batches_exte::evaluate(&[]).is_err(), "RR-0266: empty input must fail for Export OTLP batches extend codec v1");
}
