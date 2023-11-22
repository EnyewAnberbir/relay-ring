//! Integration test for `RR-0776` (basic).
//! Extended: Export OTLP batches extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0776_export_otlp_batches_exte_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x13];
    let first = relayring::capabilities::rr_0776_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0776: Extended: Export OTLP batches extend codec v11");
    let second = relayring::capabilities::rr_0776_export_otlp_batches_exte_extended::evaluate(fixture).expect("RR-0776: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0776: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0776: window consumes the whole buffer");
}
