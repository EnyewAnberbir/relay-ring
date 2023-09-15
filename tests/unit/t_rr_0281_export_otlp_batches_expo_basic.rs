//! Integration test for `RR-0281` (basic).
//! Export OTLP batches export adapter v16 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0281_export_otlp_batches_expo_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1e, 0x20];
    let first = relayring::capabilities::rr_0281_export_otlp_batches_expo::evaluate(fixture).expect("RR-0281: Export OTLP batches export adapter v16");
    let second = relayring::capabilities::rr_0281_export_otlp_batches_expo::evaluate(fixture).expect("RR-0281: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0281: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0281: window consumes the whole buffer");
}
