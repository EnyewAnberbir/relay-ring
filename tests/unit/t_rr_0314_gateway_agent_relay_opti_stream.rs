//! Integration test for `RR-0314` (stream).
//! Gateway agent relay optimize registry v14 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0314_gateway_agent_relay_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x41];
    let direct = relayring::capabilities::rr_0314_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0314: direct Gateway agent relay optimize registry v14");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0314_gateway_agent_relay_opti::evaluate(&copied).expect("RR-0314: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0314: stream path must consume input");
}
