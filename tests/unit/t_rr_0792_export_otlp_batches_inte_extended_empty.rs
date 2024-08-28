//! Integration test for `RR-0792` (empty).
//! Extended: Export OTLP batches integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0792_export_otlp_batches_inte_extended_empty() {
    assert!(relayring::capabilities::rr_0792_export_otlp_batches_inte_extended::evaluate(&[]).is_err(), "RR-0792: empty input must fail for Extended: Export OTLP batches integrate validator v27");
}
