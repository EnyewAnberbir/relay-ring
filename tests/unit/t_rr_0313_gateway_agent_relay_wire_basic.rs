//! Integration test for `RR-0313` (basic).
//! Gateway agent relay wire planner v13 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0313_gateway_agent_relay_wire_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x40];
    let first = relayring::capabilities::rr_0313_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0313: Gateway agent relay wire planner v13");
    let second = relayring::capabilities::rr_0313_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0313: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0313: RLRG journal header should be consumed");
    assert!(first.ok, "RR-0313: clean fixture should pass validation");
    assert_eq!(first.severity, 0);
}
