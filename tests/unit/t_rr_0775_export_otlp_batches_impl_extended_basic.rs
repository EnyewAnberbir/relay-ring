//! Integration test for `RR-0775` (basic).
//! Extended: Export OTLP batches implement pipeline v10 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0775_export_otlp_batches_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x12];
    let first = relayring::capabilities::rr_0775_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0775: Extended: Export OTLP batches implement pipeline v10");
    let second = relayring::capabilities::rr_0775_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0775: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0775: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0775: window consumes the whole buffer");
}
