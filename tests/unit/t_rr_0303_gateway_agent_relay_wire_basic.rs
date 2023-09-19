//! Integration test for `RR-0303` (basic).
//! Gateway agent relay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0303_gateway_agent_relay_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let first = relayring::capabilities::rr_0303_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0303: Gateway agent relay wire planner v3");
    let second = relayring::capabilities::rr_0303_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0303: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0303: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0303: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
