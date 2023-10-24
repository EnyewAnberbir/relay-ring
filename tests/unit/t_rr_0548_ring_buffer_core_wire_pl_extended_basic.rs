//! Integration test for `RR-0548` (basic).
//! Extended: Ring buffer core wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0548_ring_buffer_core_wire_pl_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x2d];
    let first = relayring::capabilities::rr_0548_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0548: Extended: Ring buffer core wire planner v23");
    let second = relayring::capabilities::rr_0548_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0548: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0548: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0548: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
