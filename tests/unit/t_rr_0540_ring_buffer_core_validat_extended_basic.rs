//! Integration test for `RR-0540` (basic).
//! Extended: Ring buffer core validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0540_ring_buffer_core_validat_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x23, 0x25];
    let first = relayring::capabilities::rr_0540_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0540: Extended: Ring buffer core validate resolver v15");
    let second = relayring::capabilities::rr_0540_ring_buffer_core_validat_extended::evaluate(fixture).expect("RR-0540: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0540: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0540: window consumes the whole buffer");
}
