//! Integration test for `RR-0285` (basic).
//! Export OTLP batches implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0285_export_otlp_batches_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0x24];
    let first = relayring::capabilities::rr_0285_export_otlp_batches_impl::evaluate(fixture).expect("RR-0285: Export OTLP batches implement pipeline v20");
    let second = relayring::capabilities::rr_0285_export_otlp_batches_impl::evaluate(fixture).expect("RR-0285: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0285: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0285: scanner should emit domain hints");
}
