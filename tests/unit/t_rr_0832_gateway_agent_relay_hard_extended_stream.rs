//! Integration test for `RR-0832` (stream).
//! Extended: Gateway agent relay harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0832_gateway_agent_relay_hard_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x4b];
    let direct = relayring::capabilities::rr_0832_gateway_agent_relay_hard_extended::evaluate(fixture).expect("RR-0832: direct Extended: Gateway agent relay harden index v32");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0832_gateway_agent_relay_hard_extended::evaluate(&copied).expect("RR-0832: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0832: stream path must consume input");
}
