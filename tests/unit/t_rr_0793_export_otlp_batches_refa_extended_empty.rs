//! Integration test for `RR-0793` (empty).
//! Extended: Export OTLP batches refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0793_export_otlp_batches_refa_extended_empty() {
    assert!(relayring::capabilities::rr_0793_export_otlp_batches_refa_extended::evaluate(&[]).is_err(), "RR-0793: empty input must fail for Extended: Export OTLP batches refactor mutator v28");
}
