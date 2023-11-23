//! Integration test for `RR-0791` (basic).
//! Extended: Export OTLP batches export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0791_export_otlp_batches_expo_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x22];
    let first = relayring::capabilities::rr_0791_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0791: Extended: Export OTLP batches export adapter v26");
    let second = relayring::capabilities::rr_0791_export_otlp_batches_expo_extended::evaluate(fixture).expect("RR-0791: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0791: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0791: window consumes the whole buffer");
}
