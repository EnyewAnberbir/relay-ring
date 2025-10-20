//! Integration test for `RR-0801` (stream).
//! Extended: Gateway agent relay extend codec v1 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0801_gateway_agent_relay_exte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x2c];
    let direct = relayring::capabilities::rr_0801_gateway_agent_relay_exte_extended::evaluate(fixture).expect("RR-0801: direct Extended: Gateway agent relay extend codec v1");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0801_gateway_agent_relay_exte_extended::evaluate(&copied).expect("RR-0801: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0801: stream path must consume input");
}
