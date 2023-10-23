//! Integration test for `RR-0542` (basic).
//! Extended: Ring buffer core integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0542_ring_buffer_core_integra_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x25, 0x27];
    let first = relayring::capabilities::rr_0542_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0542: Extended: Ring buffer core integrate validator v17");
    let second = relayring::capabilities::rr_0542_ring_buffer_core_integra_extended::evaluate(fixture).expect("RR-0542: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0542: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0542: window consumes the whole buffer");
}
