//! Integration test for `RR-0333` (roundtrip).
//! Gateway agent relay wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0333_gateway_agent_relay_wire_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x54];
    let a = relayring::capabilities::rr_0333_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0333 first pass");
    let b = relayring::capabilities::rr_0333_gateway_agent_relay_wire::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
