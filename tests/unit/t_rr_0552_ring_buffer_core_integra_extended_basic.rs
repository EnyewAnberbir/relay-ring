//! Integration test for `RR-0552` (basic).
//! Extended: Ring buffer core integrate validator v27 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0552_ring_buffer_core_integra_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x31];
    let first = relayring::capabilities::rr_0552_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0552: Extended: Ring buffer core integrate validator v27");
    let second = relayring::capabilities::rr_0552_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0552: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0552: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0552: window consumes the whole buffer");
}
