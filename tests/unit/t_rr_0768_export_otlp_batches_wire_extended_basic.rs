//! Integration test for `RR-0768` (basic).
//! Extended: Export OTLP batches wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0768_export_otlp_batches_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x0b];
    let first = relayring::capabilities::rr_0768_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0768: Extended: Export OTLP batches wire planner v3");
    let second = relayring::capabilities::rr_0768_export_otlp_batches_wire_extended::evaluate(fixture).expect("RR-0768: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0768: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0768: window consumes the whole buffer");
}
