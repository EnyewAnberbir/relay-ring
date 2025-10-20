//! Integration test for `RR-0802` (stream).
//! Extended: Gateway agent relay harden index v2 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0802_gateway_agent_relay_hard_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2b, 0x2d];
    let direct = relayring::capabilities::rr_0802_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0802: direct Extended: Gateway agent relay harden index v2");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0802_gateway_agent_relay_hard_extended::evaluate(&copied).expect("RR-0802: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0802: stream path must consume input");
}
