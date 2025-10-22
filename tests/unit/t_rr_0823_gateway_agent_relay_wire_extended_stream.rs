//! Integration test for `RR-0823` (stream).
//! Extended: Gateway agent relay wire planner v23 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0823_gateway_agent_relay_wire_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42];
    let direct = relayring::capabilities::rr_0823_gateway_agent_relay_wire_extended::evaluate(fixture).expect("RR-0823: direct Extended: Gateway agent relay wire planner v23");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0823_gateway_agent_relay_wire_extended::evaluate(&copied).expect("RR-0823: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0823: stream path must consume input");
}
