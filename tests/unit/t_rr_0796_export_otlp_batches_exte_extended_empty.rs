//! Integration test for `RR-0796` (empty).
//! Extended: Export OTLP batches extend codec v31 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0796_export_otlp_batches_exte_extended_empty() {
    assert!(relayring::capabilities::rr_0796_export_otlp_batches_exte_extended::evaluate(&[]).is_err(), "RR-0796: empty input must fail for Extended: Export OTLP batches extend codec v31");
}
