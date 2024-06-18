//! Integration test for `RR-0293` (empty).
//! Export OTLP batches refactor mutator v28 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0293_export_otlp_batches_refa_empty() {
    assert!(relayring::capabilities::rr_0293_export_otlp_batches_refa::evaluate(&[]).is_err(), "RR-0293: empty input must fail for Export OTLP batches refactor mutator v28");
}
