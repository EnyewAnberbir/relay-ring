//! Integration test for `RR-0297` (basic).
//! Export OTLP batches harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0297_export_otlp_batches_hard_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let first = relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(fixture).expect("RR-0297: Export OTLP batches harden index v32");
    let second = relayring::capabilities::rr_0297_export_otlp_batches_hard::evaluate(fixture).expect("RR-0297: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0297: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0297: window consumes the whole buffer");
}
