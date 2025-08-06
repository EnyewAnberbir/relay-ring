//! Integration test for `RR-0303` (stream).
//! Gateway agent relay wire planner v3 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0303_gateway_agent_relay_wire_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x34, 0x36];
    let direct = relayring::capabilities::rr_0303_gateway_agent_relay_wire::evaluate(fixture).expect("RR-0303: direct Gateway agent relay wire planner v3");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0303_gateway_agent_relay_wire::evaluate(&copied).expect("RR-0303: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0303: stream path must consume input");
}
