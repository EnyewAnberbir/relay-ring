//! Integration test for `RR-0772` (empty).
//! Extended: Export OTLP batches integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0772_export_otlp_batches_inte_extended_empty() {
    assert!(relayring::capabilities::rr_0772_export_otlp_batches_inte_extended::evaluate(&[]).is_err(), "RR-0772: empty input must fail for Extended: Export OTLP batches integrate validator v7");
}
