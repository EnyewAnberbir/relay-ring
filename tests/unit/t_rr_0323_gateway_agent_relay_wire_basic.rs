//! Integration test for `RR-0323` (basic).
//! Gateway agent relay wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0323_gateway_agent_relay_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x4a];
    let first = relayring::capabilities::rr_0323_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0323: Gateway agent relay wire planner v23");
    let second = relayring::capabilities::rr_0323_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0323: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0323: RLRG journal header should be consumed");
    assert_eq!(first.findings, fixture.len(), "RR-0323: stats visits every byte");
}
