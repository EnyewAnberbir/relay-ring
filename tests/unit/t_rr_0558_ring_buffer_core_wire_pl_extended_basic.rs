//! Integration test for `RR-0558` (basic).
//! Extended: Ring buffer core wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0558_ring_buffer_core_wire_pl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let first = relayring::capabilities::rr_0558_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0558: Extended: Ring buffer core wire planner v33");
    let second = relayring::capabilities::rr_0558_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0558: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0558: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0558: stats visits every byte");
}
