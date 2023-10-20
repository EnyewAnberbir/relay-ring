//! Integration test for `RR-0536` (basic).
//! Extended: Ring buffer core extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0536_ring_buffer_core_extend_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let first = relayring::capabilities::rr_0536_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0536: Extended: Ring buffer core extend codec v11");
    let second = relayring::capabilities::rr_0536_ring_buffer_core_extend_extended::evaluate(fixture).expect("RR-0536: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0536: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0536: window consumes the whole buffer");
}
