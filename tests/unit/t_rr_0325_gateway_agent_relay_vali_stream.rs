//! Integration test for `RR-0325` (stream).
//! Gateway agent relay validate resolver v25 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0325_gateway_agent_relay_vali_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4a, 0x4c];
    let direct = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(fixture).expect("RR-0325: direct Gateway agent relay validate resolver v25");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0325_gateway_agent_relay_vali::evaluate(&copied).expect("RR-0325: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0325: stream path must consume input");
}
