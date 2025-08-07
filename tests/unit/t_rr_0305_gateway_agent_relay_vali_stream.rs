//! Integration test for `RR-0305` (stream).
//! Gateway agent relay validate resolver v5 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0305_gateway_agent_relay_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x38];
    let direct = relayring::capabilities::rr_0305_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0305: direct Gateway agent relay validate resolver v5");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0305_gateway_agent_relay_vali::evaluate(&copied).expect("RR-0305: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0305: stream path must consume input");
}
