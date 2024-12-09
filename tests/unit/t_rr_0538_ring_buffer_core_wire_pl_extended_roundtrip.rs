//! Integration test for `RR-0538` (roundtrip).
//! Extended: Ring buffer core wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0538_ring_buffer_core_wire_pl_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23];
    let a = relayring::capabilities::rr_0538_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("RR-0538 first pass");
    let b = relayring::capabilities::rr_0538_ring_buffer_core_wire_pl_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
