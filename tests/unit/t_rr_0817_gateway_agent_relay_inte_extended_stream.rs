//! Integration test for `RR-0817` (stream).
//! Extended: Gateway agent relay integrate validator v17 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0817_gateway_agent_relay_inte_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3a, 0x3c];
    let direct = relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(fixture).expect("RR-0817: direct Extended: Gateway agent relay integrate validator v17");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0817_gateway_agent_relay_inte_extended::evaluate(&copied).expect("RR-0817: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0817: stream path must consume input");
}
