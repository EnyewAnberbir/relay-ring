//! Integration test for `RR-0789` (empty).
//! Extended: Export OTLP batches optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0789_export_otlp_batches_opti_extended_empty() {
    assert!(relayring::capabilities::rr_0789_export_otlp_batches_opti_extended::evaluate(&[]).is_err(), "RR-0789: empty input must fail for Extended: Export OTLP batches optimize registry v24");
}
