//! Integration test for `RR-0803` (basic).
//! Extended: Gateway agent relay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0803_gateway_agent_relay_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2c, 0x2e];
    let first = relayring::capabilities::rr_0803_gateway_agent_relay_wire_extended::evaluate(fixture).expect("RR-0803: Extended: Gateway agent relay wire planner v3");
    let second = relayring::capabilities::rr_0803_gateway_agent_relay_wire_extended::evaluate(fixture).expect("RR-0803: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0803: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0803: stats visits every byte");
}
