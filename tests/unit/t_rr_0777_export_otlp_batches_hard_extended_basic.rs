//! Integration test for `RR-0777` (basic).
//! Extended: Export OTLP batches harden index v12 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0777_export_otlp_batches_hard_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x14];
    let first = relayring::capabilities::rr_0777_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0777: Extended: Export OTLP batches harden index v12");
    let second = relayring::capabilities::rr_0777_export_otlp_batches_hard_extended::evaluate(fixture).expect("RR-0777: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0777: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0777: window consumes the whole buffer");
}
