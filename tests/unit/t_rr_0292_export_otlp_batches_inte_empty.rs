//! Integration test for `RR-0292` (empty).
//! Export OTLP batches integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0292_export_otlp_batches_inte_empty() {
    assert!(relayring::capabilities::rr_0292_export_otlp_batches_inte::evaluate(&[]).is_err(), "RR-0292: empty input must fail for Export OTLP batches integrate validator v27");
}
