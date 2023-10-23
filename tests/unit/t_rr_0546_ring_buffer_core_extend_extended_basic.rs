//! Integration test for `RR-0546` (basic).
//! Extended: Ring buffer core extend codec v21 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0546_ring_buffer_core_extend_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x2b];
    let first = relayring::capabilities::rr_0546_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0546: Extended: Ring buffer core extend codec v21");
    let second = relayring::capabilities::rr_0546_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0546: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0546: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0546: window consumes the whole buffer");
}
