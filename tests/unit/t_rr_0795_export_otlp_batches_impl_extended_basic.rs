//! Integration test for `RR-0795` (basic).
//! Extended: Export OTLP batches implement pipeline v30 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0795_export_otlp_batches_impl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x26];
    let first = relayring::capabilities::rr_0795_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0795: Extended: Export OTLP batches implement pipeline v30");
    let second = relayring::capabilities::rr_0795_export_otlp_batches_impl_extended::evaluate(fixture).expect("RR-0795: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0795: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0795: window consumes the whole buffer");
}
