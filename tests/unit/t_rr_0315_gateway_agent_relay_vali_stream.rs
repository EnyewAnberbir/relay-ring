//! Integration test for `RR-0315` (stream).
//! Gateway agent relay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0315_gateway_agent_relay_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x42];
    let direct = relayring::capabilities::rr_0315_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0315: direct Gateway agent relay validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0315_gateway_agent_relay_vali::evaluate(&copied).expect("RR-0315: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0315: stream path must consume input");
}
