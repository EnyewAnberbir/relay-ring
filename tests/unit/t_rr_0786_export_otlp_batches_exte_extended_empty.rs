//! Integration test for `RR-0786` (empty).
//! Extended: Export OTLP batches extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0786_export_otlp_batches_exte_extended_empty() {
    assert!(relayring::capabilities::rr_0786_export_otlp_batches_exte_extended::evaluate(&[]).is_err(), "RR-0786: empty input must fail for Extended: Export OTLP batches extend codec v21");
}
