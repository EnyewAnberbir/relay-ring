//! Integration test for `RR-0766` (empty).
//! Extended: Export OTLP batches extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0766_export_otlp_batches_exte_extended_empty() {
    assert!(relayring::capabilities::rr_0766_export_otlp_batches_exte_extended::evaluate(&[]).is_err(), "RR-0766: empty input must fail for Extended: Export OTLP batches extend codec v1");
}
