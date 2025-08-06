//! Integration test for `RR-0301` (stream).
//! Gateway agent relay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0301_gateway_agent_relay_exte_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x32, 0x34];
    let direct = relayring::capabilities::rr_0301_gateway_agent_relay_exte::evaluate(fixture).expect("RR-0301: direct Gateway agent relay extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0301_gateway_agent_relay_exte::evaluate(&copied).expect("RR-0301: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0301: stream path must consume input");
}
