//! Integration test for `RR-0790` (empty).
//! Extended: Export OTLP batches validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0790_export_otlp_batches_vali_extended_empty() {
    assert!(relayring::capabilities::rr_0790_export_otlp_batches_vali_extended::evaluate(&[]).is_err(), "RR-0790: empty input must fail for Extended: Export OTLP batches validate resolver v25");
}
