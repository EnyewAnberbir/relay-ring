//! Integration test for `RR-0273` (empty).
//! Export OTLP batches refactor mutator v8 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0273_export_otlp_batches_refa_empty() {
    assert!(relayring::capabilities::rr_0273_export_otlp_batches_refa::evaluate(&[]).is_err(), "RR-0273: empty input must fail for Export OTLP batches refactor mutator v8");
}
