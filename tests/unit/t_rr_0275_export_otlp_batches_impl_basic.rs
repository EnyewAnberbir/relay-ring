//! Integration test for `RR-0275` (basic).
//! Export OTLP batches implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0275_export_otlp_batches_impl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let first = relayring::capabilities::rr_0275_export_otlp_batches_impl::evaluate(fixture).expect("RR-0275: Export OTLP batches implement pipeline v10");
    let second = relayring::capabilities::rr_0275_export_otlp_batches_impl::evaluate(fixture).expect("RR-0275: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0275: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0275: scanner should emit domain hints");
}
