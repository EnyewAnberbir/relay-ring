//! Integration test for `RR-0311` (stream).
//! Gateway agent relay extend codec v11 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0311_gateway_agent_relay_exte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3e];
    let direct = relayring::capabilities::rr_0311_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0311: direct Gateway agent relay extend codec v11");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0311_gateway_agent_relay_exte::evaluate(&copied).expect("RR-0311: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0311: stream path must consume input");
}
