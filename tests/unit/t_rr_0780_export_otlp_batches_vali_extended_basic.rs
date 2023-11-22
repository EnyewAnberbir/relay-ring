//! Integration test for `RR-0780` (basic).
//! Extended: Export OTLP batches validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0780_export_otlp_batches_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x17];
    let first = relayring::capabilities::rr_0780_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0780: Extended: Export OTLP batches validate resolver v15");
    let second = relayring::capabilities::rr_0780_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0780: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0780: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0780: window consumes the whole buffer");
}
