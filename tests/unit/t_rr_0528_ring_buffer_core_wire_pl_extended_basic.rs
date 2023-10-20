//! Integration test for `RR-0528` (basic).
//! Extended: Ring buffer core wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0528_ring_buffer_core_wire_pl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x17, 0x19];
    let first = relayring::capabilities::rr_0528_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0528: Extended: Ring buffer core wire planner v3");
    let second = relayring::capabilities::rr_0528_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0528: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0528: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0528: window consumes the whole buffer");
}
