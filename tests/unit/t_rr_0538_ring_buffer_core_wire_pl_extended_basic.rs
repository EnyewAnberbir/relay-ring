//! Integration test for `RR-0538` (basic).
//! Extended: Ring buffer core wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0538_ring_buffer_core_wire_pl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let first = relayring::capabilities::rr_0538_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0538: Extended: Ring buffer core wire planner v13");
    let second = relayring::capabilities::rr_0538_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0538: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0538: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0538: window consumes the whole buffer");
}
