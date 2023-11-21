//! Integration test for `RR-0767` (basic).
//! Extended: Export OTLP batches harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0767_export_otlp_batches_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x0a];
    let first = relayring::capabilities::rr_0767_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0767: Extended: Export OTLP batches harden index v2");
    let second = relayring::capabilities::rr_0767_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0767: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0767: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0767: window consumes the whole buffer");
}
