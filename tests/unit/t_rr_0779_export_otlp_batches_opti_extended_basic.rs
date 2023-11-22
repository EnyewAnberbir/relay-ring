//! Integration test for `RR-0779` (basic).
//! Extended: Export OTLP batches optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0779_export_otlp_batches_opti_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, 0x16];
    let first = relayring::capabilities::rr_0779_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0779: Extended: Export OTLP batches optimize registry v14");
    let second = relayring::capabilities::rr_0779_export_otlp_batches_opti_extended::evaluate(fixture).expect("RR-0779: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0779: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0779: window consumes the whole buffer");
}
