//! Integration test for `RR-0783` (empty).
//! Extended: Export OTLP batches refactor mutator v18 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0783_export_otlp_batches_refa_extended_empty() {
    assert!(relayring::capabilities::rr_0783_export_otlp_batches_refa_extended::evaluate(&[]).is_err(), "RR-0783: empty input must fail for Extended: Export OTLP batches refactor mutator v18");
}
