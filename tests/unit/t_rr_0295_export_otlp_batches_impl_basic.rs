//! Integration test for `RR-0295` (basic).
//! Export OTLP batches implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0295_export_otlp_batches_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x2e];
    let first = relayring::capabilities::rr_0295_export_otlp_batches_impl::evaluate(fixture).expect("RR-0295: Export OTLP batches implement pipeline v30");
    let second = relayring::capabilities::rr_0295_export_otlp_batches_impl::evaluate(fixture).expect("RR-0295: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0295: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0295: stats visits every byte");
}
