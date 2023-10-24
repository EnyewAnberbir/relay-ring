//! Integration test for `RR-0557` (basic).
//! Extended: Ring buffer core harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0557_ring_buffer_core_harden_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let first = relayring::capabilities::rr_0557_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0557: Extended: Ring buffer core harden index v32");
    let second = relayring::capabilities::rr_0557_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0557: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0557: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0557: window consumes the whole buffer");
}
