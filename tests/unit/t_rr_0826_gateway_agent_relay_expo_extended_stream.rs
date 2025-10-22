//! Integration test for `RR-0826` (stream).
//! Extended: Gateway agent relay export adapter v26 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0826_gateway_agent_relay_expo_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43, 0x45];
    let direct = relayring::capabilities::rr_0826_gateway_agent_relay_expo_extended::evaluate(fixture).expect("RR-0826: direct Extended: Gateway agent relay export adapter v26");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0826_gateway_agent_relay_expo_extended::evaluate(&copied).expect("RR-0826: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0826: stream path must consume input");
}
