//! Integration test for `RR-0527` (basic).
//! Extended: Ring buffer core harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0527_ring_buffer_core_harden_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x16, 0x18];
    let first = relayring::capabilities::rr_0527_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0527: Extended: Ring buffer core harden index v2");
    let second = relayring::capabilities::rr_0527_ring_buffer_core_harden_extended::evaluate(fixture).expect("RR-0527: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0527: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0527: window consumes the whole buffer");
}
