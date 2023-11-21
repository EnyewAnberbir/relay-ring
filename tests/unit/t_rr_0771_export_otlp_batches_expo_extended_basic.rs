//! Integration test for `RR-0771` (basic).
//! Extended: Export OTLP batches export adapter v6 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0771_export_otlp_batches_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x0e];
    let first = relayring::capabilities::rr_0771_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0771: Extended: Export OTLP batches export adapter v6");
    let second = relayring::capabilities::rr_0771_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0771: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0771: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0771: window consumes the whole buffer");
}
