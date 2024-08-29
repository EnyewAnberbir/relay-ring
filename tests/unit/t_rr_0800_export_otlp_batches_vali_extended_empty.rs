//! Integration test for `RR-0800` (empty).
//! Extended: Export OTLP batches validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0800_export_otlp_batches_vali_extended_empty() {
    assert!(relayring::capabilities::rr_0800_export_otlp_batches_vali_extended::evaluate(&[]).is_err(), "RR-0800: empty input must fail for Extended: Export OTLP batches validate resolver v35");
}
