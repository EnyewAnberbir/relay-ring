//! Integration test for `RR-0333` (basic).
//! Gateway agent relay wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0333_gateway_agent_relay_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x54];
    let first = relayring::capabilities::rr_0333_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0333: Gateway agent relay wire planner v33");
    let second = relayring::capabilities::rr_0333_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0333: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0333: RLRG journal header should be consumed");
    assert!(first.findings > 0, "RR-0333: scanner should emit domain hints");
}
