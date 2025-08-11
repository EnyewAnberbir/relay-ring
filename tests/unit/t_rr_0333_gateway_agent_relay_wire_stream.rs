//! Integration test for `RR-0333` (stream).
//! Gateway agent relay wire planner v33 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0333_gateway_agent_relay_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x54];
    let direct = relayring::capabilities::rr_0333_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0333: direct Gateway agent relay wire planner v33");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0333_gateway_agent_relay_wire::evaluate(&copied).expect("RR-0333: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0333: stream path must consume input");
}
