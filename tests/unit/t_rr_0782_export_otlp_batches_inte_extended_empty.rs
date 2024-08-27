//! Integration test for `RR-0782` (empty).
//! Extended: Export OTLP batches integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0782_export_otlp_batches_inte_extended_empty() {
    assert!(relayring::capabilities::rr_0782_export_otlp_batches_inte_extended::evaluate(&[]).is_err(), "RR-0782: empty input must fail for Extended: Export OTLP batches integrate validator v17");
}
