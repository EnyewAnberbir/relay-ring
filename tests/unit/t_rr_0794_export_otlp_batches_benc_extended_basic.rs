//! Integration test for `RR-0794` (basic).
//! Extended: Export OTLP batches benchmark reporter v29 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0794_export_otlp_batches_benc_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let first = relayring::capabilities::rr_0794_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0794: Extended: Export OTLP batches benchmark reporter v29");
    let second = relayring::capabilities::rr_0794_export_otlp_batches_benc_extended::evaluate(fixture).expect("RR-0794: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0794: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0794: window consumes the whole buffer");
}
