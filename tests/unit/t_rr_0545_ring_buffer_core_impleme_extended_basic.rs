//! Integration test for `RR-0545` (basic).
//! Extended: Ring buffer core implement pipeline v20 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0545_ring_buffer_core_impleme_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x28, 0x2a];
    let first = relayring::capabilities::rr_0545_ring_buffer_core_impleme_extended::evaluate(fixture).expect("RR-0545: Extended: Ring buffer core implement pipeline v20");
    let second = relayring::capabilities::rr_0545_ring_buffer_core_impleme_extended::evaluate(fixture).expect("RR-0545: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0545: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0545: window consumes the whole buffer");
}
