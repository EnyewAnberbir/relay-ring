//! Integration test for `RR-0529` (basic).
//! Extended: Ring buffer core optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0529_ring_buffer_core_optimiz_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x1a];
    let first = relayring::capabilities::rr_0529_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0529: Extended: Ring buffer core optimize registry v4");
    let second = relayring::capabilities::rr_0529_ring_buffer_core_optimiz_extended::evaluate(fixture).expect("RR-0529: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0529: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0529: window consumes the whole buffer");
}
