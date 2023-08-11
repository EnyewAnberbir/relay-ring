//! Integration test for `RR-0028` (basic).
//! Ring buffer core wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0028_ring_buffer_core_wire_pl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1f, 0x21];
    let first = relayring::capabilities::rr_0028_ring_buffer_core_wire_pl::evaluate(fixture).expect("RR-0028: Ring buffer core wire planner v3");
    let second = relayring::capabilities::rr_0028_ring_buffer_core_wire_pl::evaluate(fixture).expect("RR-0028: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0028: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0028: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
