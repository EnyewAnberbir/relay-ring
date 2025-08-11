//! Integration test for `RR-0334` (stream).
//! Gateway agent relay optimize registry v34 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0334_gateway_agent_relay_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x53, 0x55];
    let direct = relayring::capabilities::rr_0334_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0334: direct Gateway agent relay optimize registry v34");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0334_gateway_agent_relay_opti::evaluate(&copied).expect("RR-0334: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0334: stream path must consume input");
}
