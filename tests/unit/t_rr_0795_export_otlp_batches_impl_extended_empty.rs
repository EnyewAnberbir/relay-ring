//! Integration test for `RR-0795` (empty).
//! Extended: Export OTLP batches implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0795_export_otlp_batches_impl_extended_empty() {
    assert!(relayring::capabilities::rr_0795_export_otlp_batches_impl_extended::evaluate(&[]).is_err(), "RR-0795: empty input must fail for Extended: Export OTLP batches implement pipeline v30");
}
