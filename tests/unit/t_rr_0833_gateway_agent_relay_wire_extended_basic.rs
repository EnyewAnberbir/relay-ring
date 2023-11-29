//! Integration test for `RR-0833` (basic).
//! Extended: Gateway agent relay wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0833_gateway_agent_relay_wire_extended_basic() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x4c];
    let first = relayring::capabilities::rr_0833_gateway_agent_relay_wire_extended::evaluate(fixture).expect("RR-0833: Extended: Gateway agent relay wire planner v33");
    let second = relayring::capabilities::rr_0833_gateway_agent_relay_wire_extended::evaluate(fixture).expect("RR-0833: replay");
    assert_eq!(first, second);
    assert!(first.consumed >= 4, "RR-0833: RLRG journal header should be consumed");
    assert_eq!(first.consumed, fixture.len(), "RR-0833: window consumes the whole buffer");
}
