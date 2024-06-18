//! Integration test for `RR-0295` (empty).
//! Export OTLP batches implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0295_export_otlp_batches_impl_empty() {
    assert!(relayring::capabilities::rr_0295_export_otlp_batches_impl::evaluate(&[]).is_err(), "RR-0295: empty input must fail for Export OTLP batches implement pipeline v30");
}
