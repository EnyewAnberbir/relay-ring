//! Integration test for `RR-0822` (stream).
//! Extended: Gateway agent relay harden index v22 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0822_gateway_agent_relay_hard_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let direct = relayring::capabilities::rr_0822_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0822: direct Extended: Gateway agent relay harden index v22");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0822_gateway_agent_relay_hard_extended::evaluate(&copied).expect("RR-0822: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0822: stream path must consume input");
}
