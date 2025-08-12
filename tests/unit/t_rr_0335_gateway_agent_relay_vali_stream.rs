//! Integration test for `RR-0335` (stream).
//! Gateway agent relay validate resolver v35 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0335_gateway_agent_relay_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x54, 0x56];
    let direct = relayring::capabilities::rr_0335_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0335: direct Gateway agent relay validate resolver v35");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0335_gateway_agent_relay_vali::evaluate(&copied).expect("RR-0335: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0335: stream path must consume input");
}
