//! Integration test for `RR-0805` (stream).
//! Extended: Gateway agent relay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0805_gateway_agent_relay_vali_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x30];
    let direct = relayring::capabilities::rr_0805_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0805: direct Extended: Gateway agent relay validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0805_gateway_agent_relay_vali_extended::evaluate(&copied).expect("RR-0805: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0805: stream path must consume input");
}
