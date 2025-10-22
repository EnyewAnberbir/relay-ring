//! Integration test for `RR-0815` (stream).
//! Extended: Gateway agent relay validate resolver v15 — RLRG relay-ring journal frame fixtures.

#[test]
fn rr_0815_gateway_agent_relay_vali_extended_stream() {
    let fixture: &[u8] = &[0x52, 0x4c, 0x52, 0x47, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x38, 0x3a];
    let direct = relayring::capabilities::rr_0815_gateway_agent_relay_vali_extended::evaluate(fixture).expect("RR-0815: direct Extended: Gateway agent relay validate resolver v15");
    let copied: Vec<u8> = fixture.to_vec();
    let buffered = relayring::capabilities::rr_0815_gateway_agent_relay_vali_extended::evaluate(&copied).expect("RR-0815: buffered replay");
    assert_eq!(direct, buffered);
    assert!(direct.consumed > 0, "RR-0815: stream path must consume input");
}
