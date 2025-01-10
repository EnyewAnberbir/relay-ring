//! Integration test for `RR-0803` (roundtrip).
//! Extended: Gateway agent relay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0803_gateway_agent_relay_wire_extended_roundtrip() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x2e];
    let a = relayring::capabilities::rr_0803_gateway_agent_relay_wire_extended::evaluate(fixture).expect("RR-0803 first pass");
    let b = relayring::capabilities::rr_0803_gateway_agent_relay_wire_extended::evaluate(fixture).expect("second pass");
    assert_eq!(a.checksum, b.checksum);
    assert_eq!(a.findings, b.findings);
    assert_eq!(a.ok, b.ok);
}
