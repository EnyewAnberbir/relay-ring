//! Integration test for `RR-0038` (basic).
//! Ring buffer core wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0038_ring_buffer_core_wire_pl_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x2b];
    let first = relayring::capabilities::rr_0038_ring_buffer_core_wire_pl::evaluate(fixture).expect("RR-0038: Ring buffer core wire planner v13");
    let second = relayring::capabilities::rr_0038_ring_buffer_core_wire_pl::evaluate(fixture).expect("RR-0038: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0038: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0038: stats visits every byte");
}
