//! Integration test for `RR-0332` (stream).
//! Gateway agent relay harden index v32 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0332_gateway_agent_relay_hard_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x51, 0x53];
    let direct = relayring::capabilities::rr_0332_gateway_agent_relay_hard::evaluate(fixture).expect("RR-0332: direct Gateway agent relay harden index v32");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0332_gateway_agent_relay_hard::evaluate(&copied).expect("RR-0332: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0332: stream path must consume input");
}
