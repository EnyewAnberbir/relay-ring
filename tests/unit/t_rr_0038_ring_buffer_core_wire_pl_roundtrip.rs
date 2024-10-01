//! Integration test for `RR-0038` (roundtrip).
//! Ring buffer core wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0038_ring_buffer_core_wire_pl_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x29, 0x2b];
    let a = relayring::capabilities::rr_0038_ring_buffer_core_wire_pl::evaluate(fixture).expect("RR-0038 first pass");
    let b = relayring::capabilities::rr_0038_ring_buffer_core_wire_pl::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
