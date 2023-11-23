//! Integration test for `RR-0790` (basic).
//! Extended: Export OTLP batches validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0790_export_otlp_batches_vali_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let first = relayring::capabilities::rr_0790_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0790: Extended: Export OTLP batches validate resolver v25");
    let second = relayring::capabilities::rr_0790_export_otlp_batches_vali_extended::evaluate(fixture).expect("RR-0790: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0790: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0790: window consumes the whole buffer");
}
