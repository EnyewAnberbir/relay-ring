//! Integration test for `RR-0824` (stream).
//! Extended: Gateway agent relay optimize registry v24 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0824_gateway_agent_relay_opti_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x43];
    let direct = relayring::capabilities::rr_0824_gateway_agent_relay_opti_extended::evaluate(fixture).expect("RR-0824: direct Extended: Gateway agent relay optimize registry v24");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0824_gateway_agent_relay_opti_extended::evaluate(&copied).expect("RR-0824: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0824: stream path must consume input");
}
