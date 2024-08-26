//! Integration test for `RR-0773` (empty).
//! Extended: Export OTLP batches refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0773_export_otlp_batches_refa_extended_empty() {
    assert!(relayring::capabilities::rr_0773_export_otlp_batches_refa_extended::evaluate(&[]).is_err(), "RR-0773: empty input must fail for Extended: Export OTLP batches refactor mutator v8");
}
