//! Integration test for `RR-0272` (empty).
//! Export OTLP batches integrate validator v7 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0272_export_otlp_batches_inte_empty() {
    assert!(relayring::capabilities::rr_0272_export_otlp_batches_inte::evaluate(&[]).is_err(), "RR-0272: empty input must fail for Export OTLP batches integrate validator v7");
}
