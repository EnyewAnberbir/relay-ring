//! Integration test for `RR-0780` (empty).
//! Extended: Export OTLP batches validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0780_export_otlp_batches_vali_extended_empty() {
    assert!(relayring::capabilities::rr_0780_export_otlp_batches_vali_extended::evaluate(&[]).is_err(), "RR-0780: empty input must fail for Extended: Export OTLP batches validate resolver v15");
}
