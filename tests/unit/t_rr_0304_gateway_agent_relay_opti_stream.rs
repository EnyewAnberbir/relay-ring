//! Integration test for `RR-0304` (stream).
//! Gateway agent relay optimize registry v4 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0304_gateway_agent_relay_opti_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x35, 0x37];
    let direct = relayring::capabilities::rr_0304_gateway_agent_relay_opti::evaluate(fixture).expect("RR-0304: direct Gateway agent relay optimize registry v4");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0304_gateway_agent_relay_opti::evaluate(&copied).expect("RR-0304: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0304: stream path must consume input");
}
