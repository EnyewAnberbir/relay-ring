//! Integration test for `RR-0769` (empty).
//! Extended: Export OTLP batches optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0769_export_otlp_batches_opti_extended_empty() {
    assert!(relayring::capabilities::rr_0769_export_otlp_batches_opti_extended::evaluate(&[]).is_err(), "RR-0769: empty input must fail for Extended: Export OTLP batches optimize registry v4");
}
