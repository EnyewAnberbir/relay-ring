//! Integration test for `RR-0798` (basic).
//! Extended: Export OTLP batches wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0798_export_otlp_batches_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x29];
    let first = relayring::capabilities::rr_0798_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0798: Extended: Export OTLP batches wire planner v33");
    let second = relayring::capabilities::rr_0798_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0798: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0798: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0798: window consumes the whole buffer");
}
